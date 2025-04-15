use bit_field::BitField;
use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub struct Crmd {
    bits: usize,
}

impl Crmd {
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn from_bits(bits: usize) -> Self {
        Self { bits }
    }

    pub fn write(bits: usize) {
        unsafe {
            asm!("csrwr {}, 0x1", in(reg) bits);
        }
    }

    pub fn read() -> Self {
        let bits: usize;
        unsafe {
            asm!("csrrd {}, 0x1", out(reg) bits);
        }
        Self::from_bits(bits)
    }

    pub fn set_bits(&mut self, range: core::ops::RangeInclusive<usize>, value: usize) {
        self.bits.set_bits(range, value);
    }

    pub fn ie(&self) -> bool {
        self.bits.get_bit(2)
    }

    pub fn clear_ie() {
        unsafe {
            let mut crmd: usize;
            // Read the CRMD register
            asm!("csrrd {0}, 0x4", out(reg) crmd);
            // Clear the IE bit (bit 2)
            crmd &= !(1 << 2);
            // Write the updated value back to CRMD
            asm!("csrwr {0}, 0x4", in(reg) crmd);
        }
    }
    
    pub fn set_ie() {
        unsafe {
            let mut crmd: usize;
            // Read the CRMD register
            asm!("csrrd {0}, 0x4", out(reg) crmd);
            // Set the IE bit (bit 2)
            crmd |= 1 << 2;
            // Write the updated value back to CRMD
            asm!("csrwr {0}, 0x4", in(reg) crmd);
        }
    }
}
