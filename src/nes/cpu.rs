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

/// Public Functions Implementation
impl Cpu {
    /// 割り込みを処理します
    pub fn interrupt(&mut self, system: &mut System, irq_type: Interrupt) {
        let is_nested_interrupt = self.read_interrupt_flag();
        // RESET, NMI以外は多重割り込みを許容しない
        if is_nested_interrupt {
            if (irq_type == Interrupt::IRQ) || (irq_type == Interrupt::BRK) {
                return;
            }
        }
        // 割り込み種類別の処理
        match irq_type {
            Interrupt::NMI   => {
                self.write_break_flag(false);
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.sp >> 8) as u8);
                self.stack_push(system, (self.sp & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            },
            Interrupt::RESET => {
                self.write_interrupt_flag(true);
            },
            Interrupt::IRQ   => {
                self.write_break_flag(false);
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.sp >> 8) as u8);
                self.stack_push(system, (self.sp & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            },
            Interrupt::BRK   => {
                self.write_break_flag(true);
                self.pc = self.pc + 1;
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.sp >> 8) as u8);
                self.stack_push(system, (self.sp & 0xff) as u8);
                self.stack_push(system, self.p);
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
}

/// Stack Control Implementation
impl Cpu {
    /// Stack Push操作を行います
    fn stack_push(&mut self, system: &mut System, data: u8) {
        // data store
        system.write_u8(self.sp as usize, data);
        // decrement
        self.sp = self.sp - 1;
    }

    /// Stack Pop操作を行います
    fn stack_pop(&mut self, system: &System) -> u8 {
        // increment
        self.sp = self.sp + 1;
        // data fetch
        let data = system.read_u8(self.sp as usize);
        return data;
    }

}

enum Addressing {
    /// アキュームレータ上の値だけで十分(0を返しとく)
    Accumulator,
    /// opcode, data
    Immediate,
    /// opcode, lower address, upper address
    Absolute,
    /// opcode, lower (upperは0x00固定)
    ZeroPage,
    /// opcode, lower+x (upperは0x00固定)
    IndexedZeroPageX,
    /// opcode, lower+y (upperは0x00固定)
    IndexedZeroPageY,
    ///opcode, lower, upper => (lower | upper << 8) + x
    IndexedAbsoluteX,
    ///opcode, lower, upper => (lower | upper << 8) + y
    IndexedAbsoluteY,
    /// 不要
    Implied,
    /// opcode, offset(符号拡張 s8) =>次の命令を示すpc+offset
    Relative,
    /// opcode, lower => (lower+x)のアドレスに格納されているデータをlower_byte, その次をupper_byteとして完成したアドレスを実行アドレスとしてfetch
    /// キャリーは無視
    IndexedIndirect,
    /// opcode, lower => lowerのアドレスに格納されているデータをupper, 次をlowerとして作った実効アドレスにyを加算してからfetch
    /// キャリーは無視
    IndirectIndexed,
    /// opcode, lower, upper, lowerとupperで作った実効アドレスでfetch
    /// 下位バイトのキャリーは無視
    AbsoluteIndirect,
}
/// Fetch and Adressing Implementation
impl Cpu {
    fn fetch_accumulator() -> u8 { return 0x0; }
    fn fetch_immediate(&self, system: &System) -> u8 {
        let data_addr = self.pc + 1;
        let data = system.read_u8(data_addr as usize);
        return data;
    }
    fn fetch_absolute(&self, system: &System) -> u8 {
        let lower_addr = self.pc + 1;
        let upper_addr = self.pc + 2;
        let lower = system.read_u8(lower_addr as usize);
        let upper = system.read_u8(upper_addr as usize);
        let addr  = (lower as u16) | ((upper as u16) << 8);
        let data  = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_zero_page(&self, system: &System) -> u8 {
        let lower_addr = self.pc + 1;
        let lower = system.read_u8(lower_addr as usize);
        let addr  = (lower as u16);
        let data  = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_indexed_zero_page_x(&self, system: &System) -> u8 {
        let lower_addr = self.pc + 1;
        let lower = system.read_u8(lower_addr as usize);
        let addr  = (lower as u16) + (self.x as u16);
        let data  = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_indexed_zero_page_y(&self, system: &System) -> u8 {
        let lower_addr = self.pc + 1;
        let lower = system.read_u8(lower_addr as usize);
        let addr  = (lower as u16) + (self.y as u16);
        let data  = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_indexed_absolute_x(&self, system: &System) -> u8 {
        let lower_addr = self.pc + 1;
        let upper_addr = self.pc + 2;
        let lower = system.read_u8(lower_addr as usize);
        let upper = system.read_u8(upper_addr as usize);
        let addr  = (lower as u16) | ((upper as u16) << 8) + (self.x as u16);
        let data  = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_indexed_absolute_y(&self, system: &System) -> u8 {
        let lower_addr = self.pc + 1;
        let upper_addr = self.pc + 2;
        let lower = system.read_u8(lower_addr as usize);
        let upper = system.read_u8(upper_addr as usize);
        let addr  = (lower as u16) | ((upper as u16) << 8) + (self.y as u16);
        let data  = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_implied() -> u8 { return 0x0; }
    fn fetch_relative(&self, system: &System) -> u8 {
        let offset_addr = self.pc + 1;
        let offset = system.read_u8(offset_addr as usize);
        let addr_signed  = ((offset as i8) as i32) + (self.pc as i32);
        assert!(addr_signed >= 0);
        assert!(addr_signed < 0x1000);
        let addr = addr_signed as u16;
        let data = system.read_u8(addr as usize);
        return data;
    }
    fn fetch_indexed_indirect(&self, system: &System) -> u8 {
        let addr1 = self.pc + 1;
        let data1 = system.read_u8(addr1 as usize);
        let addr2 = (data1 as u16) + (self.x as u16);
        let data2_lower = system.read_u8(addr2 as usize);
        let data2_upper = system.read_u8((addr2 + 1) as usize);
        let addr3 = (data2_lower as u16) | ((data2_upper as u16) << 8);
        let data3 = system.read_u8(addr3 as usize);
        return data3;
    }
    fn fetch_indirect_indexed(&self, system: &System) -> u8 {
        let addr1 = self.pc + 1;
        let data1 = system.read_u8(addr1 as usize);
        let addr2 = (data1 as u16);
        // TODO: upper, lower逆かもしれない
        let data2_upper = system.read_u8(addr2 as usize);
        let data2_lower = system.read_u8((addr2 + 1) as usize);
        let addr3 = (data2_lower as u16) | ((data2_upper as u16) << 8) + (self.y as u16);
        let data3 = system.read_u8(addr3 as usize);
        return data3;
    }
    fn fetch_absolute_indirect(&self, system: &System) -> u8 {
        let addr1_lower = self.pc + 1;
        let addr1_upper = self.pc + 2;
        let addr1 = (addr1_lower) + (addr1_upper << 8);
        let data1 = system.read_u8(addr1 as usize);
        let addr2 = (data1 as u16);
        let data2_lower = system.read_u8(addr2 as usize);
        let data2_upper = system.read_u8((addr2 + 1) as usize);
        let addr3 = (data2_lower as u16) | ((data2_upper as u16) << 8);
        let data3 = system.read_u8(addr3 as usize);
        return data3;
    }
}

/// Processor Status Flag Implementation
impl Cpu {
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