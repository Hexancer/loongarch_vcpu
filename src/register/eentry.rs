use bit_field::BitField;
use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub struct Eentry {
    bits: usize,
}

impl Eentry {
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn from_bits(bits: usize) -> Self {
        Self { bits }
    }

    pub fn write(bits: usize) {
        unsafe {
            asm!("csrwr {}, 0xc", in(reg) bits);
        }
    }

    pub fn read() -> Self {
        let bits: usize;
        unsafe {
            asm!("csrrd {}, 0xc", out(reg) bits);
        }
        Self::from_bits(bits)
    }

    pub fn set_bits(&mut self, range: core::ops::RangeInclusive<usize>, value: usize) {
        self.bits.set_bits(range, value);
    }
}
