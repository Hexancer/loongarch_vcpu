use axaddrspace::{GuestPhysAddr, HostPhysAddr, MappingFlags};
use axerrno::AxResult;
use axvcpu::{AxVCpuExitReason, AxVCpuHal};
use core::arch::asm;
use crate::LoongArchVCpuCreateConfig;

use crate::regs::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct VCpuConfig {}

#[derive(Default)]
pub struct LoongArchVCpu<H: AxVCpuHal> {
    regs: VmCpuRegisters,
    _marker: core::marker::PhantomData<H>,
}

impl<H: AxVCpuHal> axvcpu::AxArchVCpu for LoongArchVCpu<H> {
    type CreateConfig = LoongArchVCpuCreateConfig;
    type SetupConfig = ();

    fn new(config: Self::CreateConfig) -> AxResult<Self> {
        let mut regs = VmCpuRegisters::default();
        // 设置通用寄存器：a0 (hartid), a1 (dtb地址)
        regs.guest_regs.gprs.set_reg(GprIndex::A0, config.hart_id);
        regs.guest_regs.gprs.set_reg(GprIndex::A1, config.dtb_addr.as_usize());

        Ok(Self {
            regs,
            _marker: core::marker::PhantomData,
        })
    }

    fn setup(&mut self, _config: Self::SetupConfig) -> AxResult {
        // 设置 guest 的 CSR 寄存器
        unsafe {
            // 设置 PRMD (特权级模式)
            core::arch::asm!(
                "li.w $r4, 0b11 << 3",  // PLV=3 (guest模式)
                "csrwr $r4, prmd"
            );
            
            // 设置 ESTAT (异常状态寄存器)
            core::arch::asm!(
                "li.w $r4, 0",  // 清除所有异常状态
                "csrwr $r4, estat"
            );
            
            // 设置 ECFG (异常配置寄存器)
            core::arch::asm!(
                "li.w $r4, (1 << 12) | (1 << 13)",  // 启用定时器和IPI中断
                "csrwr $r4, ecfg"
            );
            
            // 设置 TICLR (定时器中断清除寄存器)
            core::arch::asm!(
                "li.w $r4, 1",  // 清除定时器中断
                "csrwr $r4, ticlr"
            );
        }
        
        // 初始化其他控制状态
        self.regs.guest_regs.era = 0;  // 异常返回地址
        self.regs.virtual_hs_csrs.hgatp = 0;  // 页表基址
        
        Ok(())
    }

    fn set_entry(&mut self, entry: GuestPhysAddr) -> AxResult {
        self.regs.guest_regs.era = entry.as_usize(); // LoongArch 使用 ERA 作为异常返回地址
        Ok(())
    }

    fn set_ept_root(&mut self, ept_root: HostPhysAddr) -> AxResult {
        self.regs.virtual_hs_csrs.hgatp = ept_root.as_usize();
        Ok(())
    }

    fn run(&mut self) -> AxResult<AxVCpuExitReason> {
        unsafe {
            // LoongArch 需要禁用中断等预处理
            _run_guest_loongarch(&mut self.regs);
        }
        self.vmexit_handler()
    }

    fn bind(&mut self) -> AxResult {
        unsafe {
            core::arch::asm!(
                "csrwr {0}, hgatp", 
                in(reg) self.regs.virtual_hs_csrs.hgatp
            );
        }
        Ok(())
    }

    fn unbind(&mut self) -> AxResult {
        Ok(())
    }

    fn set_gpr(&mut self, index: usize, val: usize) {
        if let Some(idx) = GprIndex::from_raw(index as u32) {
            self.set_gpr_from_gpr_index(idx, val);
        }
    }
}

impl<H: AxVCpuHal> LoongArchVCpu<H> {
    pub fn get_gpr(&self, index: GprIndex) -> usize {
        self.regs.guest_regs.gprs.reg(index)
    }

    pub fn set_gpr_from_gpr_index(&mut self, index: GprIndex, val: usize) {
        self.regs.guest_regs.gprs.set_reg(index, val);
    }

    pub fn advance_pc(&mut self, instr_len: usize) {
        self.regs.guest_regs.era += instr_len;
    }

    pub fn regs(&mut self) -> &mut VmCpuRegisters {
        &mut self.regs
    }

    fn vmexit_handler(&mut self) -> AxResult<AxVCpuExitReason> {
        let estat = self.regs.trap_csrs.estat;
        let ecode = (estat >> 16) & 0x3ff;
        let esubcode = (estat >> 22) & 0x3f;
        let era = self.regs.guest_regs.era;

        match ecode {
            // 处理系统调用
            0x100 => {
                self.advance_pc(4);
                Ok(AxVCpuExitReason::Hypercall {
                    nr: self.regs.guest_regs.gprs.reg(GprIndex::A7) as _,
                    args: [
                        self.regs.guest_regs.gprs.reg(GprIndex::A0) as _,
                        self.regs.guest_regs.gprs.reg(GprIndex::A1) as _,
                        self.regs.guest_regs.gprs.reg(GprIndex::A2) as _,
                        self.regs.guest_regs.gprs.reg(GprIndex::A3) as _,
                        self.regs.guest_regs.gprs.reg(GprIndex::A4) as _,
                        self.regs.guest_regs.gprs.reg(GprIndex::A5) as _,
                    ],
                })
            }
            // 处理定时器中断
            0x400 => {
                Ok(AxVCpuExitReason::Nothing)
            }
            // 处理页错误
            0x800 => {
                let badv = self.regs.trap_csrs.badv;
                Ok(AxVCpuExitReason::NestedPageFault {
                    addr: GuestPhysAddr::from(badv),
                    access_flags: MappingFlags::empty(),
                })
            }
            _ => {
                panic!(
                    "Unhandled trap: ecode={:#x}, esubcode={:#x}, era={:#x}",
                    ecode, esubcode, era
                );
            }
        }
    }
}

// LoongArch 特有的汇编入口
unsafe extern "C" fn _run_guest_loongarch(state: *mut VmCpuRegisters) {
    unsafe {
        core::arch::asm!(
            // 保存 host 状态
            "csrwr {save3}, {LOONGARCH_CSR_SAVE3}",
            // 加载 guest 状态
            "ld.d $r4, {state_ptr}, 0",  // 加载 era
            "csrwr $r4, {era}", //#define LOONGARCH_CSR_ERA		0x6	/* ERA */
            "ld.d $r4, {state_ptr}, 8",  // 加载 pgd (LoongArch的页表基址寄存器)
            "csrwr $r4, 0x1b",  // PGD CSR编号 (LOONGARCH_CSR_PGD)
            // 进入 guest 执行
            "ertn",
            // 退出时恢复 host 状态
            "ld.d $r4, {state_ptr}, 8",  // 恢复 pgd
            "csrwr $r4, 0x1b",  // PGD CSR编号 (LOONGARCH_CSR_PGD)
            era = const 0x6,
            LOONGARCH_CSR_SAVE3 = const 0x33,
            save3 = in(reg) (state as usize + core::mem::size_of::<VmCpuRegisters>()),
            state_ptr = in(reg) state,
            options(noreturn)
        );
    }
}

impl<H: AxVCpuHal> LoongArchVCpu<H> {
    fn handle_timer_interrupt(&mut self) -> AxResult<AxVCpuExitReason> {
        // 处理定时器中断
        unsafe {
            core::arch::asm!("csrwr $r0, 0x41"); // 清除定时器中断
        }
        Ok(AxVCpuExitReason::Nothing)
    }

    fn handle_ipi(&mut self) -> AxResult<AxVCpuExitReason> {
        // 处理核间中断
        Ok(AxVCpuExitReason::Nothing)  // LoongArch使用Nothing代替Interrupt
    }

    fn flush_tlb(&mut self) {
        unsafe {
            asm!("invtlb 0, $r0, $r0"); // 刷新TLB
        }
    }
}
