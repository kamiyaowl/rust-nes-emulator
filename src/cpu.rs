use super::interface::*;
use super::system::System;

pub const CPU_FREQ: u32 = 1790000;
pub const NMI_READ_LOWER: u16 = 0xfffa;
pub const NMI_READ_UPPER: u16 = 0xfffb;
pub const RESET_READ_LOWER: u16 = 0xfffc;
pub const RESET_READ_UPPER: u16 = 0xfffd;
pub const IRQ_READ_LOWER: u16 = 0xfffe;
pub const IRQ_READ_UPPER: u16 = 0xffff;
pub const BRK_READ_LOWER: u16 = 0xfffe;
pub const BRK_READ_UPPER: u16 = 0xffff;

#[derive(PartialEq, Eq)]
pub enum Interrupt {
    NMI,
    RESET,
    IRQ,
    BRK,
}

#[derive(Clone)]
pub struct Cpu {
    /// Accumulator
    pub a: u8,
    /// Index Register
    pub x: u8,
    /// Index Register
    pub y: u8,
    /// Program Counter
    pub pc: u16,
    /// Stack Pointer
    /// 上位8bitは0x1固定
    pub sp: u16,
    /// Processor Status Register
    /// Negative, oVerflow, Reserved(1固定), Break, Decimal, Interrupt, Zero, Carry
    pub p: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            p: 0,
        }
    }
}

impl EmulateControl for Cpu {
    fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0;
        self.sp = 0x01fd;
        self.p = 0x34;
    }
}

/// Control Functions Implementation
impl Cpu {
    /// プログラムカウンタを指定した数進めます
    pub fn increment_pc(&mut self, incr: u16) {
        self.pc = self.pc + incr;
    }
    /// Stack Push操作を行います
    pub fn stack_push(&mut self, system: &mut System, data: u8) {
        // data store
        system.write_u8(self.sp, data, false);
        // decrement
        self.sp = self.sp - 1;
    }

    /// Stack Pop操作を行います
    pub fn stack_pop(&mut self, system: &mut System) -> u8 {
        // increment
        self.sp = self.sp + 1;
        // data fetch
        system.read_u8(self.sp, false)
    }
    /// 割り込みを処理します
    pub fn interrupt(&mut self, system: &mut System, irq_type: Interrupt) {
        let is_nested_interrupt = self.read_interrupt_flag();
        // RESET, NMI以外は多重割り込みを許容しない
        if is_nested_interrupt && (irq_type == Interrupt::IRQ) || (irq_type == Interrupt::BRK) {
            return;
        }
        // 割り込み種類別の処理
        match irq_type {
            Interrupt::NMI => {
                self.write_break_flag(false);
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            }
            Interrupt::RESET => {
                self.write_interrupt_flag(true);
            }
            Interrupt::IRQ => {
                self.write_break_flag(false);
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            }
            Interrupt::BRK => {
                self.write_break_flag(true);
                self.pc = self.pc + 1;
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            }
        }
        // Program Counterの書き換え
        let lower_addr = match irq_type {
            Interrupt::NMI => NMI_READ_LOWER,
            Interrupt::RESET => RESET_READ_LOWER,
            Interrupt::IRQ => IRQ_READ_LOWER,
            Interrupt::BRK => BRK_READ_LOWER,
        };
        let upper_addr = match irq_type {
            Interrupt::NMI => NMI_READ_UPPER,
            Interrupt::RESET => RESET_READ_UPPER,
            Interrupt::IRQ => IRQ_READ_UPPER,
            Interrupt::BRK => BRK_READ_UPPER,
        };

        let lower = system.read_u8(lower_addr, false);
        let upper = system.read_u8(upper_addr, false);
        self.pc = (lower as u16) | ((upper as u16) << 8);
    }
}
