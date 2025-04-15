use bit_field::BitField;

impl_define_csr!(Gstat, "GSTAT");
impl_read_csr!(0x50, Gstat);

impl Gstat {
    pub fn pgm(&self) -> bool {
        self.bits.get_bit(1)
    }
    pub fn gidbits(&self) -> usize {
        self.bits.get_bits(4..=9)
    }
    pub fn gid(&self) -> usize {
        self.bits.get_bits(16..=23)
    }
}

pub fn set_gid(gid: usize) {
    set_csr_loong_bits!(0x50, 16..=23, gid);
}

pub fn set_pgm(pgm: bool) {
    set_csr_loong_bit!(0x50, 1, pgm);
}


// use bit_field::BitField;

// impl_define_csr!(Gstat, "GSTAT");
// impl_read_csr!(0x50, Gstat);
// impl_write_csr!(0x50, Gstat);
// impl_set_clear_csr!(Gstat);

// impl Gstat {
//     /// Page Global Map status
//     pub fn pgm(&self) -> bool {
//         self.bits.get_bit(1)
//     }
    
//     /// Guest ID bits width
//     pub fn gidbits(&self) -> usize {
//         self.bits.get_bits(4..=9)
//     }
    
//     /// Current Guest ID
//     pub fn gid(&self) -> usize {
//         self.bits.get_bits(16..=23)
//     }
    
//     /// Virtualization enabled
//     pub fn hv(&self) -> bool {
//         self.bits.get_bit(0)
//     }
    
//     /// Timer interrupt pending
//     pub fn tip(&self) -> bool {
//         self.bits.get_bit(2)
//     }
    
//     /// External interrupt pending
//     pub fn eip(&self) -> bool {
//         self.bits.get_bit(3)
//     }
// }

// /// Set Guest ID
// pub fn set_gid(gid: usize) {
//     set_csr_loong_bits!(0x50, 16..=23, gid);
// }

// /// Set Page Global Map status
// pub fn set_pgm(pgm: bool) {
//     set_csr_loong_bit!(0x50, 1, pgm);
// }

// /// Enable/disable virtualization
// pub fn set_hv(hv: bool) {
//     set_csr_loong_bit!(0x50, 0, hv);
// }

// /// Clear timer interrupt pending
// pub fn clear_tip() {
//     clear_csr_loong_bit!(0x50, 2);
// }

// /// Clear external interrupt pending
// pub fn clear_eip() {
//     clear_csr_loong_bit!(0x50, 3);
// }
