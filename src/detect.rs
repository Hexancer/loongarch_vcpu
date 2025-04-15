//! Detect instruction sets (ISA extensions) by trap-and-return procedure

use core::arch::{asm, naked_asm};
use crate::register::{
    ecfg::Ecfg,
    estat::{Exception, Estat, Trap},
    eentry::Eentry,
    crmd::Crmd,
};

/// Detect if hypervisor extension exists on current core environment
pub fn detect_h_extension() -> bool {
    let ans = with_detect_trap(0, || unsafe {
        asm!("csrrd  {}, 0x680", out(reg) _, options(nomem, nostack)); // 0x680 => hgatp
    });
    ans != 2
}

#[inline]
fn with_detect_trap(param: usize, f: impl FnOnce()) -> usize {
    let (ie, eentry, tp) = unsafe { init_detect_trap(param) };
    f();
    unsafe { restore_detect_trap(ie, eentry, tp) }
}

extern "C" fn rust_detect_trap(trap_frame: &mut TrapFrame) {
    trap_frame.tp = trap_frame.estat.bits();
    match trap_frame.estat.cause() {
        Exception::IPE => {  // IPE is the IllegalInstruction code in LoongArch
            trap_frame.era = trap_frame.era.wrapping_add(4);
        }
        _ => unreachable!(),
    }
}

#[inline]
unsafe fn init_detect_trap(param: usize) -> (bool, Eentry, usize) {
    let stored_ie = Crmd::read().ie();
    Crmd::clear_ie();
    
    let stored_eentry = Eentry::read();
    let trap_addr = on_detect_trap as usize;
    let stored_tp: usize;
    
    asm!(
        "move  {}, $tp",
        "move  $tp, {}",
        out(reg) stored_tp,
        in(reg) param,
        options(nomem, nostack)
    );
    Eentry::write(trap_addr);
    
    (stored_ie, stored_eentry, stored_tp)
}

#[inline]
unsafe fn restore_detect_trap(ie: bool, eentry: Eentry, tp: usize) -> usize {
    let ans: usize;
    asm!(
        "move  {}, $tp",
        "move  $tp, {}",
        out(reg) ans,
        in(reg) tp,
        options(nomem, nostack)
    );
    
    Eentry::write(eentry.bits());
    if ie {
        Crmd::set_ie();
    }
    ans
}

#[repr(C)]
struct TrapFrame {
    ra: usize,
    tp: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    crmd: usize,
    era: usize,
    estat: Estat,
    badv: usize,
}

#[naked]
unsafe extern "C" fn on_detect_trap() -> ! {
    unsafe {
        naked_asm!(
            ".p2align 2",
            "addi.d  $sp, $sp, -8*21",
            "st.d    $ra, $sp, 0*8",
            "st.d    $tp, $sp, 1*8",
            "st.d    $a0, $sp, 2*8",
            "st.d    $a1, $sp, 3*8",
            "st.d    $a2, $sp, 4*8",
            "st.d    $a3, $sp, 5*8",
            "st.d    $a4, $sp, 6*8",
            "st.d    $a5, $sp, 7*8",
            "st.d    $a6, $sp, 8*8",
            "st.d    $a7, $sp, 9*8",
            "st.d    $t0, $sp, 10*8",
            "st.d    $t1, $sp, 11*8",
            "st.d    $t2, $sp, 12*8",
            "st.d    $t3, $sp, 13*8",
            "st.d    $t4, $sp, 14*8",
            "st.d    $t5, $sp, 15*8",
            "st.d    $t6, $sp, 16*8",
            "csrrd   $t0, 0x0",  // crmd
            "st.d    $t0, $sp, 17*8",
            "csrrd   $t1, 0x6",  // era
            "st.d    $t1, $sp, 18*8",
            "csrrd   $t2, 0x5",  // estat
            "st.d    $t2, $sp, 19*8",
            "csrrd   $t3, 0x7",  // badv
            "st.d    $t3, $sp, 20*8",
            "move    $a0, $sp",
            "bl      rust_detect_trap",
            "ld.d    $t0, $sp, 17*8",
            "csrwr   $t0, 0x0",
            "ld.d    $t1, $sp, 18*8",
            "csrwr   $t1, 0x6",
            "ld.d    $t2, $sp, 19*8",
            "csrwr   $t2, 0x5",
            "ld.d    $t3, $sp, 20*8",
            "csrwr   $t3, 0x7",
            "ld.d    $ra, $sp, 0*8",
            "ld.d    $tp, $sp, 1*8",
            "ld.d    $a0, $sp, 2*8",
            "ld.d    $a1, $sp, 3*8",
            "ld.d    $a2, $sp, 4*8",
            "ld.d    $a3, $sp, 5*8",
            "ld.d    $a4, $sp, 6*8",
            "ld.d    $a5, $sp, 7*8",
            "ld.d    $a6, $sp, 8*8",
            "ld.d    $a7, $sp, 9*8",
            "ld.d    $t0, $sp, 10*8",
            "ld.d    $t1, $sp, 11*8",
            "ld.d    $t2, $sp, 12*8",
            "ld.d    $t3, $sp, 13*8",
            "ld.d    $t4, $sp, 14*8",
            "ld.d    $t5, $sp, 15*8",
            "ld.d    $t6, $sp, 16*8",
            "addi.d  $sp, $sp, 8*21",
            "ertn"
        )
    }
}
