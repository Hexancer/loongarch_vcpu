use core::mem::size_of;
use memoffset::offset_of;
use crate::regs::*;

// LoongArch 寄存器偏移量定义
#[allow(dead_code)]
const fn hyp_gpr_offset(index: GprIndex) -> usize {
    offset_of!(VmCpuRegisters, hyp_regs)
        + offset_of!(HypervisorCpuState, gprs)
        + (index as usize) * size_of::<u64>()
}

#[allow(dead_code)]
const fn guest_gpr_offset(index: GprIndex) -> usize {
    offset_of!(VmCpuRegisters, guest_regs)
        + offset_of!(GuestCpuState, gprs)
        + (index as usize) * size_of::<u64>()
}

// LoongArch CSR 偏移量宏
#[allow(unused_macros)]
macro_rules! hyp_csr_offset {
    ($reg:tt) => {
        offset_of!(VmCpuRegisters, hyp_regs) + offset_of!(HypervisorCpuState, $reg)
    };
}

#[allow(unused_macros)]
macro_rules! guest_csr_offset {
    ($reg:tt) => {
        offset_of!(VmCpuRegisters, guest_regs) + offset_of!(GuestCpuState, $reg)
    };
}

core::arch::global_asm!(
    include_str!("trap_la.S"),
    // hyp_ra = const hyp_gpr_offset(GprIndex::RA),
    // hyp_tp = const hyp_gpr_offset(GprIndex::TP),
    // hyp_a0 = const hyp_gpr_offset(GprIndex::A0),
    // hyp_a1 = const hyp_gpr_offset(GprIndex::A1),
    // hyp_a2 = const hyp_gpr_offset(GprIndex::A2),
    // hyp_a3 = const hyp_gpr_offset(GprIndex::A3),
    // hyp_a4 = const hyp_gpr_offset(GprIndex::A4),
    // hyp_a5 = const hyp_gpr_offset(GprIndex::A5),
    // hyp_a6 = const hyp_gpr_offset(GprIndex::A6),
    // hyp_a7 = const hyp_gpr_offset(GprIndex::A7),
    // hyp_sp = const hyp_gpr_offset(GprIndex::SP),
    
    // hyp_crmd = const hyp_csr_offset!(crmd),
    // hyp_prmd = const hyp_csr_offset!(prmd),
    // hyp_euen = const hyp_csr_offset!(euen),
    // hyp_ectl = const hyp_csr_offset!(ectl),
    // hyp_estat = const hyp_csr_offset!(estat),
    // hyp_era = const hyp_csr_offset!(era),
    
    // guest_ra = const guest_gpr_offset(GprIndex::RA),
    // guest_tp = const guest_gpr_offset(GprIndex::TP),
    // guest_a0 = const guest_gpr_offset(GprIndex::A0),
    // guest_a1 = const guest_gpr_offset(GprIndex::A1),
    // guest_a2 = const guest_gpr_offset(GprIndex::A2),
    // guest_a3 = const guest_gpr_offset(GprIndex::A3),
    // guest_a4 = const guest_gpr_offset(GprIndex::A4),
    // guest_a5 = const guest_gpr_offset(GprIndex::A5),
    // guest_a6 = const guest_gpr_offset(GprIndex::A6),
    // guest_a7 = const guest_gpr_offset(GprIndex::A7),
    // guest_sp = const guest_gpr_offset(GprIndex::SP),

    // guest_crmd = const guest_csr_offset!(crmd),
    // guest_prmd = const guest_csr_offset!(prmd),
    // guest_euen = const guest_csr_offset!(euen),
    // guest_ectl = const guest_csr_offset!(ectl),
    // guest_estat = const guest_csr_offset!(estat),
    // guest_era = const guest_csr_offset!(era),
);
