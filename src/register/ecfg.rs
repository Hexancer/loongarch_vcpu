use bit_field::BitField;
use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub struct Ecfg {
    bits: usize,
}

impl Ecfg {
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn from_bits(bits: usize) -> Self {
        Self { bits }
    }

    pub fn write(bits: usize) {
        unsafe {
            asm!("csrwr {}, 0x4", in(reg) bits);
        }
    }

    pub fn read() -> Self {
        let bits: usize;
        unsafe {
            asm!("csrrd {}, 0x4", out(reg) bits);
        }
        Self::from_bits(bits)
    }

    pub fn set_bits(&mut self, range: core::ops::RangeInclusive<usize>, value: usize) {
        self.bits.set_bits(range, value);
    }
}
