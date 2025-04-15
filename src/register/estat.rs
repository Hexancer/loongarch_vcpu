use bit_field::BitField;
use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub enum Exception {
    Int = 0,
    PIL = 1,
    PIS = 2,
    PIF = 3,
    PME = 4,
    PNR = 5,
    PNX = 6,
    PP = 7,
    ADEF = 8,
    ADEM = 9,
    ALE = 10,
    BCE = 11,
    SYS = 12,
    BRK = 13,
    INE = 14,
    IPE = 15,
    FPD = 16,
    SXD = 17,
    ASXD = 18,
    FPE = 19,
    VFPE = 20,
    WPEF = 21,
    TLBR = 22,
}

#[derive(Debug, Clone, Copy)]
pub enum Trap {
    Interrupt = 0,
    Exception = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Estat {
    bits: usize,
}

impl Estat {
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn from_bits(bits: usize) -> Self {
        Self { bits }
    }

    pub fn write(bits: usize) {
        unsafe {
            asm!("csrwr {}, 0x5", in(reg) bits);
        }
    }

    pub fn read() -> Self {
        let bits: usize;
        unsafe {
            asm!("csrrd {}, 0x5", out(reg) bits);
        }
        Self::from_bits(bits)
    }

    pub fn set_bits(&mut self, range: core::ops::RangeInclusive<usize>, value: usize) {
        self.bits.set_bits(range, value);
    }

    pub fn cause(&self) -> Exception {
        match self.bits.get_bits(0..=4) {
            0 => Exception::Int,
            1 => Exception::PIL,
            2 => Exception::PIS,
            3 => Exception::PIF,
            4 => Exception::PME,
            5 => Exception::PNR,
            6 => Exception::PNX,
            7 => Exception::PP,
            8 => Exception::ADEF,
            9 => Exception::ADEM,
            10 => Exception::ALE,
            11 => Exception::BCE,
            12 => Exception::SYS,
            13 => Exception::BRK,
            14 => Exception::INE,
            15 => Exception::IPE,
            16 => Exception::FPD,
            17 => Exception::SXD,
            18 => Exception::ASXD,
            19 => Exception::FPE,
            20 => Exception::VFPE,
            21 => Exception::WPEF,
            22 => Exception::TLBR,
            _ => panic!("Invalid exception code"),
        }
    }
}
