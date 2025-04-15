#![no_std]
#![feature(naked_functions)]
#![doc = include_str!("../README.md")]

// #[macro_use]
extern crate log;

pub mod register;

mod consts;
mod detect;
mod percpu;
mod regs;
mod trap;
mod vcpu;

pub use self::percpu::LoongArchPerCpu;
pub use self::vcpu::LoongArchVCpu;
pub use detect::detect_h_extension as has_hardware_support;

/// Extension ID for hypercall, defined by ourselves.
/// `0x4C`, `0x41`, `0x56` is "LAV" in ASCII, short for "LoongArch VCPU".
pub const EID_LAV: usize = 0x4C4156;

/// Configuration for creating a new `LoongArchVCpu`
#[derive(Clone, Debug)]
pub struct LoongArchVCpuCreateConfig {
    /// The ID of the vCPU, default to `0`.
    pub hart_id: usize,
    /// The physical address of the device tree blob.
    /// Default to `0x8000_0000`.
    pub dtb_addr: axaddrspace::GuestPhysAddr,
}

impl Default for LoongArchVCpuCreateConfig {
    fn default() -> Self {
        Self {
            hart_id: 0,
            dtb_addr: axaddrspace::GuestPhysAddr::from_usize(0x8000_0000),
        }
    }
}
