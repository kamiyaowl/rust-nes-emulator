use super::system::System;
use super::interface::{SystemBus, EmulateControl};

const NMI_READ_LOWER:   usize = 0xfffa;
const NMI_READ_UPPER:   usize = 0xfffb;
const RESET_READ_LOWER: usize = 0xfffc;
const RESET_READ_UPPER: usize = 0xfffd;
const IRQ_READ_LOWER:   usize = 0xfffe;
const IRQ_READ_UPPER:   usize = 0xffff;
const BRK_READ_LOWER:   usize = 0xfffe;
const BRK_READ_UPPER:   usize = 0xffff;

#[derive(PartialEq, Eq)]
pub enum Interrupt {
    NMI, RESET, IRQ, BRK,
}

pub struct Cpu {
    /// Accumulator
    pub a : u8,
    /// Index Register
    pub x : u8,
    /// Index Register
    pub y : u8,
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    /// 上位8bitは0x1固定
    pub sp: u16, 
    /// Processor Status Register
    /// Negative, oVerflow, Reserved(1固定), Break, Decimal, Interrupt, Zero, Carry
    pub p  : u8,
}

impl EmulateControl for Cpu {
    fn reset(&mut self){
        self.a  = 0;
        self.x  = 0;
        self.y  = 0;
        self.pc = 0;
        // Stack Pointerの上位byteは固定値
        self.sp = 0x0100;
        // StatusはReservedは立てっぱなしにする
        self.p  = 0;
        self.write_reserved_flag(true);
    }
    fn store(&self, read_callback: fn(usize, u8)) {
        // レジスタダンプを連番で取得する(little endian)
        read_callback(0, self.a);
        read_callback(1, self.x);
        read_callback(2, self.y);
        read_callback(3, (self.pc & 0xff) as u8);
        read_callback(4, ((self.pc >> 8) & 0xff) as u8);
        read_callback(5, (self.sp & 0xff) as u8);
        read_callback(6, ((self.sp >> 8) & 0xff) as u8);
        read_callback(7, self.p);
    }
    fn restore(&mut self, write_callback: fn(usize) -> u8) {
        // store通りに復元してあげる
        self.a  = write_callback(0);
        self.x  = write_callback(1);
        self.y  = write_callback(2);
        self.pc = (write_callback(3) as u16) | ((write_callback(4) as u16) << 8);
        self.sp = (write_callback(5) as u16) | ((write_callback(6) as u16) << 8);
        self.p  = write_callback(7);
    }
}

impl Cpu {
    /// 割り込みを処理します
    pub fn interrupt(&mut self, irq_type: Interrupt, system: &System) {
        let is_nested_interrupt = self.read_interrupt_flag();
        // RESET, NMI以外は多重割り込みを許容しない
        if (is_nested_interrupt) {
            if (irq_type == Interrupt::IRQ) || (irq_type == Interrupt::BRK) {
                return;
            }
        }
        // 割り込み種類別の処理
        match irq_type {
            Interrupt::NMI   => {
                self.write_break_flag(false);
                // TODO: PCのUpper, Lower, Status RegisterをStackに格納する
                self.write_interrupt_flag(true);
            },
            Interrupt::RESET => {
                self.write_interrupt_flag(true);
            },
            Interrupt::IRQ   => {
                self.write_break_flag(false);
                // TODO: PCのUpper, Lower, Status RegisterをStackに格納する
                self.write_interrupt_flag(true);
            },
            Interrupt::BRK   => {
                self.write_break_flag(true);
                self.pc = self.pc + 1;
                // TODO: PCのUpper, Lower, Status RegisterをStackに格納する
                self.write_interrupt_flag(true);
            },
        }
        // Program Counterの書き換え
        let lower_addr = match irq_type {
                Interrupt::NMI   => NMI_READ_LOWER,
                Interrupt::RESET => RESET_READ_LOWER,
                Interrupt::IRQ   => IRQ_READ_LOWER,
                Interrupt::BRK   => BRK_READ_LOWER,
            };
        let upper_addr = match irq_type {
                Interrupt::NMI   => NMI_READ_UPPER,
                Interrupt::RESET => RESET_READ_UPPER,
                Interrupt::IRQ   => IRQ_READ_UPPER,
                Interrupt::BRK   => BRK_READ_UPPER,
            };
        
        let lower = system.read_u8(lower_addr);
        let upper = system.read_u8(upper_addr);
        self.pc = (lower as u16) | ((upper as u16) << 8);
    }

    fn write_negative_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x80u8;
        } else {
            self.p = self.p & (!0x80u8);
        }
    }
    fn write_overflow_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x40u8;
        } else {
            self.p = self.p & (!0x40u8);
        }
    }
    fn write_reserved_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x20u8;
        } else {
            self.p = self.p & (!0x20u8);
        }
    }
    fn write_break_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x10u8;
        } else {
            self.p = self.p & (!0x10u8);
        }
    }
    fn write_decimal_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x08u8;
        } else {
            self.p = self.p & (!0x08u8);
        }
    }
    fn write_interrupt_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x04u8;
        } else {
            self.p = self.p & (!0x04u8);
        }
    }
    fn write_zero_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x02u8;
        } else {
            self.p = self.p & (!0x02u8);
        }
    }
    fn write_carry_flag(&mut self, is_active: bool) {
        if is_active {
            self.p = self.p | 0x01u8;
        } else {
            self.p = self.p & (!0x01u8);
        }
    }
    fn read_negative_flag(&self)  -> bool { return (self.p & 0x80u8) != 0x00u8; }
    fn read_overflow_flag(&self)  -> bool { return (self.p & 0x40u8) != 0x00u8; }
    fn read_reserved_flag(&self)  -> bool { return (self.p & 0x20u8) != 0x00u8; }
    fn read_break_flag(&self)     -> bool { return (self.p & 0x10u8) != 0x00u8; }
    fn read_decimal_flag(&self)   -> bool { return (self.p & 0x08u8) != 0x00u8; }
    fn read_interrupt_flag(&self) -> bool { return (self.p & 0x04u8) != 0x00u8; }
    fn read_zero_flag(&self)      -> bool { return (self.p & 0x02u8) != 0x00u8; }
    fn read_carry_flag(&self)     -> bool { return (self.p & 0x01u8) != 0x00u8; }

}