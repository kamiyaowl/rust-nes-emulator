use super::cpu::*;

/// Processor Status Flag Implementation
impl Cpu {
    pub fn write_negative_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x80u8;
        } else {
            self.p = self.p & (!0x80u8);
        }
    }
    pub fn write_overflow_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x40u8;
        } else {
            self.p = self.p & (!0x40u8);
        }
    }
    pub fn write_reserved_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x20u8;
        } else {
            self.p = self.p & (!0x20u8);
        }
    }
    pub fn write_break_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x10u8;
        } else {
            self.p = self.p & (!0x10u8);
        }
    }
    pub fn write_decimal_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x08u8;
        } else {
            self.p = self.p & (!0x08u8);
        }
    }
    pub fn write_interrupt_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x04u8;
        } else {
            self.p = self.p & (!0x04u8);
        }
    }
    pub fn write_zero_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x02u8;
        } else {
            self.p = self.p & (!0x02u8);
        }
    }
    pub fn write_carry_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x01u8;
        } else {
            self.p = self.p & (!0x01u8);
        }
    }
    pub fn read_negative_flag(&self) -> bool {
        (self.p & 0x80u8) == 0x80u8
    }
    pub fn read_overflow_flag(&self) -> bool {
        (self.p & 0x40u8) == 0x40u8
    }
    pub fn read_reserved_flag(&self) -> bool {
        (self.p & 0x20u8) == 0x20u8
    }
    pub fn read_break_flag(&self) -> bool {
        (self.p & 0x10u8) == 0x10u8
    }
    pub fn read_decimal_flag(&self) -> bool {
        (self.p & 0x08u8) == 0x08u8
    }
    pub fn read_interrupt_flag(&self) -> bool {
        (self.p & 0x04u8) == 0x04u8
    }
    pub fn read_zero_flag(&self) -> bool {
        (self.p & 0x02u8) == 0x02u8
    }
    pub fn read_carry_flag(&self) -> bool {
        (self.p & 0x01u8) == 0x01u8
    }
}
