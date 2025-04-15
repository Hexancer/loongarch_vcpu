use core::marker::PhantomData;

use axerrno::{AxError, AxResult};
use axvcpu::{AxArchPerCpu, AxVCpuHal};

// use crate::consts::traps;
use crate::has_hardware_support;

/// LoongArch per-CPU state.
pub struct LoongArchPerCpu<H: AxVCpuHal> {
    _marker: PhantomData<H>,
}

impl<H: AxVCpuHal> AxArchPerCpu for LoongArchPerCpu<H> {
    fn new(_cpu_id: usize) -> AxResult<Self> {
        unsafe {
            setup_csrs();
        }

        Ok(Self {
            _marker: PhantomData,
        })
    }

    fn is_enabled(&self) -> bool {
        unimplemented!()
    }

    fn hardware_enable(&mut self) -> AxResult<()> {
        if has_hardware_support() {
            Ok(())
        } else {
            Err(AxError::Unsupported)
        }
    }

    fn hardware_disable(&mut self) -> AxResult<()> {
        unimplemented!()
    }
}

/// Initialize LoongArch CSRs to a reasonable state.
unsafe fn setup_csrs() {
    unsafe {
        // 设置异常委托寄存器 (ECFG)
        core::arch::asm!(
            "li.w $r4, {ecfg_val}",
            "csrwr $r4, {ecfg}",
            ecfg_val = const (1 << 12) | (1 << 13), // 启用定时器和IPI中断
            ecfg = const 0x4, // ECFG CSR编号 (LOONGARCH_CSR_ECFG)
        );

        // 清除所有中断状态
        core::arch::asm!(
            "li.w $r4, 0",
            "csrwr $r4, {estat}",  // 清除异常状态
            "li.w $r4, 1",
            "csrwr $r4, {ticlr}",
            estat = const 0x5,  // 清除定时器中断
            ticlr = const 0x44, // TICLR CSR编号 (LOONGARCH_CSR_TICLR)
        );

        // 设置特权模式 (PRMD)
        core::arch::asm!(
            "li.w $r4, 0b11 << 3",  // PLV=3 (guest模式)
            "csrwr $r4, {prmd}",  // 设置PRMD寄存器
            prmd = const 0x1, // PRMD CSR编号 (LOONGARCH_CSR_PRMD)
        );

        // 启用中断
        core::arch::asm!(
            "li.w $r4, 0x800",  // 启用外部中断
            "csrwr $r4, {ecfg}", // 设置ECFG寄存器
            ecfg = const 0x4, // ECFG CSR编号 (LOONGARCH_CSR_ECFG)
        );
    }
}
