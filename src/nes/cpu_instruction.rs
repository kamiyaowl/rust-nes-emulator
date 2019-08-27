use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

#[derive(Copy, Clone)]
pub enum Opcode {
    // binary op
    ADC, SBC, AND, EOR, ORA,  
    // shift/rotate
    ASL, LSR, ROL, ROR, 
    // inc/dec
    INC, INX, INY, DEC, DEX, DEY, 
    // load/store
    LDA, LDX, LDY, STA, STX, STY, 
    // set/clear flag
    SEC, SED, SEI, CLC, CLD, CLI, CLV, 
    // compare
    CMP, CPX, CPY, 
    // jump return
    JMP, JSR, RTI, RTS, 
    // branch
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS, 
    // push/pop
    PHA, PHP, PLA, PLP, 
    // transfer
    TAX, TAY, TSX, TXA, TXS, TYA,
    // other
    BRK, BIT, NOP,
}

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate, 
    Absolute,  
    ZeroPage,  
    ZeroPageX, 
    ZeroPageY, 
    AbsoluteX, 
    AbsoluteY, 
    Relative, 
    Indirect,  
    IndirectX,
    IndirectY,
}
#[derive(Copy, Clone)]
/// (data, cyc)
pub struct Operand(u16, u8);

#[derive(Copy, Clone)]
pub struct Instruction(Opcode, AddressingMode);

// TODO: 別ファイルに追い出し
impl Instruction {
        /// romのコードを命令に変換します
    pub fn from(inst_code: u8) -> Instruction {
        match inst_code {
            /* *************** binary op ***************  */
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            0x65 => Instruction(Opcode::ADC, AddressingMode::ZeroPage),
            0x75 => Instruction(Opcode::ADC, AddressingMode::ZeroPageX),
            0x6d => Instruction(Opcode::ADC, AddressingMode::Absolute),
            0x7d => Instruction(Opcode::ADC, AddressingMode::AbsoluteX),
            0x79 => Instruction(Opcode::ADC, AddressingMode::AbsoluteY),
            0x61 => Instruction(Opcode::ADC, AddressingMode::IndirectX),
            0x71 => Instruction(Opcode::ADC, AddressingMode::IndirectY),

            0xe9 => Instruction(Opcode::SBC, AddressingMode::Immediate),
            0xe5 => Instruction(Opcode::SBC, AddressingMode::ZeroPage),
            0xf5 => Instruction(Opcode::SBC, AddressingMode::ZeroPageX),
            0xed => Instruction(Opcode::SBC, AddressingMode::Absolute),
            0xfd => Instruction(Opcode::SBC, AddressingMode::AbsoluteX),
            0xf9 => Instruction(Opcode::SBC, AddressingMode::AbsoluteY),
            0xe1 => Instruction(Opcode::SBC, AddressingMode::IndirectX),
            0xf1 => Instruction(Opcode::SBC, AddressingMode::IndirectY),

            0x29 => Instruction(Opcode::AND, AddressingMode::Immediate),
            0x25 => Instruction(Opcode::AND, AddressingMode::ZeroPage),
            0x35 => Instruction(Opcode::AND, AddressingMode::ZeroPageX),
            0x2d => Instruction(Opcode::AND, AddressingMode::Absolute),
            0x3d => Instruction(Opcode::AND, AddressingMode::AbsoluteX),
            0x39 => Instruction(Opcode::AND, AddressingMode::AbsoluteY),
            0x21 => Instruction(Opcode::AND, AddressingMode::IndirectX),
            0x31 => Instruction(Opcode::AND, AddressingMode::IndirectY),

            0x49 => Instruction(Opcode::EOR, AddressingMode::Immediate),
            0x45 => Instruction(Opcode::EOR, AddressingMode::ZeroPage),
            0x55 => Instruction(Opcode::EOR, AddressingMode::ZeroPageX),
            0x4d => Instruction(Opcode::EOR, AddressingMode::Absolute),
            0x5d => Instruction(Opcode::EOR, AddressingMode::AbsoluteX),
            0x59 => Instruction(Opcode::EOR, AddressingMode::AbsoluteY),
            0x41 => Instruction(Opcode::EOR, AddressingMode::IndirectX),
            0x51 => Instruction(Opcode::EOR, AddressingMode::IndirectY),

            0x09 => Instruction(Opcode::ORA, AddressingMode::Immediate),
            0x05 => Instruction(Opcode::ORA, AddressingMode::ZeroPage),
            0x15 => Instruction(Opcode::ORA, AddressingMode::ZeroPageX),
            0x0d => Instruction(Opcode::ORA, AddressingMode::Absolute),
            0x1d => Instruction(Opcode::ORA, AddressingMode::AbsoluteX),
            0x19 => Instruction(Opcode::ORA, AddressingMode::AbsoluteY),
            0x01 => Instruction(Opcode::ORA, AddressingMode::IndirectX),
            0x11 => Instruction(Opcode::ORA, AddressingMode::IndirectY),

            /* *************** shift/rotate op ***************  */
            0x0a => Instruction(Opcode::ASL, AddressingMode::Accumulator),
            0x06 => Instruction(Opcode::ASL, AddressingMode::ZeroPage),
            0x16 => Instruction(Opcode::ASL, AddressingMode::ZeroPageX),
            0x0e => Instruction(Opcode::ASL, AddressingMode::Absolute),
            0x1e => Instruction(Opcode::ASL, AddressingMode::AbsoluteX),

            0x4a => Instruction(Opcode::LSR, AddressingMode::Accumulator),
            0x46 => Instruction(Opcode::LSR, AddressingMode::ZeroPage),
            0x56 => Instruction(Opcode::LSR, AddressingMode::ZeroPageX),
            0x4e => Instruction(Opcode::LSR, AddressingMode::Absolute),
            0x5e => Instruction(Opcode::LSR, AddressingMode::AbsoluteX),

            0x2a => Instruction(Opcode::ROL, AddressingMode::Accumulator),
            0x26 => Instruction(Opcode::ROL, AddressingMode::ZeroPage),
            0x36 => Instruction(Opcode::ROL, AddressingMode::ZeroPageX),
            0x2e => Instruction(Opcode::ROL, AddressingMode::Absolute),
            0x3e => Instruction(Opcode::ROL, AddressingMode::AbsoluteX),

            0x6a => Instruction(Opcode::ROR, AddressingMode::Accumulator),
            0x66 => Instruction(Opcode::ROR, AddressingMode::ZeroPage),
            0x76 => Instruction(Opcode::ROR, AddressingMode::ZeroPageX),
            0x6e => Instruction(Opcode::ROR, AddressingMode::Absolute),
            0x7e => Instruction(Opcode::ROR, AddressingMode::AbsoluteX),

            /* *************** inc/dec op ***************  */
            0xe6 => Instruction(Opcode::INC, AddressingMode::ZeroPage),
            0xf6 => Instruction(Opcode::INC, AddressingMode::ZeroPageX),
            0xee => Instruction(Opcode::INC, AddressingMode::Absolute),
            0xfe => Instruction(Opcode::INC, AddressingMode::AbsoluteX),

            0xe8 => Instruction(Opcode::INX, AddressingMode::Implied),
            0xc8 => Instruction(Opcode::INY, AddressingMode::Implied),
            
            0xc6 => Instruction(Opcode::DEC, AddressingMode::ZeroPage),
            0xd6 => Instruction(Opcode::DEC, AddressingMode::ZeroPageX),
            0xce => Instruction(Opcode::DEC, AddressingMode::Absolute),
            0xde => Instruction(Opcode::DEC, AddressingMode::AbsoluteX),

            0xca => Instruction(Opcode::DEX, AddressingMode::Implied),
            0x88 => Instruction(Opcode::DEY, AddressingMode::Implied),
            
            /* *************** load/store op ***************  */
            0xa9 => Instruction(Opcode::LDA, AddressingMode::Immediate),
            0xa5 => Instruction(Opcode::LDA, AddressingMode::ZeroPage),
            0xb5 => Instruction(Opcode::LDA, AddressingMode::ZeroPageX),
            0xad => Instruction(Opcode::LDA, AddressingMode::Absolute),
            0xbd => Instruction(Opcode::LDA, AddressingMode::AbsoluteX),
            0xb9 => Instruction(Opcode::LDA, AddressingMode::AbsoluteY),
            0xa1 => Instruction(Opcode::LDA, AddressingMode::IndirectX),
            0xb1 => Instruction(Opcode::LDA, AddressingMode::IndirectY),
            
            0xa2 => Instruction(Opcode::LDX, AddressingMode::Immediate),
            0xa6 => Instruction(Opcode::LDX, AddressingMode::ZeroPage),
            0xb6 => Instruction(Opcode::LDX, AddressingMode::ZeroPageY),
            0xae => Instruction(Opcode::LDX, AddressingMode::Absolute),
            0xbe => Instruction(Opcode::LDX, AddressingMode::AbsoluteY),
            
            0xa0 => Instruction(Opcode::LDY, AddressingMode::Immediate),
            0xa4 => Instruction(Opcode::LDY, AddressingMode::ZeroPage),
            0xb4 => Instruction(Opcode::LDY, AddressingMode::ZeroPageX),
            0xac => Instruction(Opcode::LDY, AddressingMode::Absolute),
            0xbc => Instruction(Opcode::LDY, AddressingMode::AbsoluteX),
            
            0x85 => Instruction(Opcode::STA, AddressingMode::ZeroPage),
            0x95 => Instruction(Opcode::STA, AddressingMode::ZeroPageX),
            0x8d => Instruction(Opcode::STA, AddressingMode::Absolute),
            0x9d => Instruction(Opcode::STA, AddressingMode::AbsoluteX),
            0x99 => Instruction(Opcode::STA, AddressingMode::AbsoluteY),
            0x81 => Instruction(Opcode::STA, AddressingMode::IndirectX),
            0x91 => Instruction(Opcode::STA, AddressingMode::IndirectY),
            
            0x86 => Instruction(Opcode::STX, AddressingMode::ZeroPage),
            0x96 => Instruction(Opcode::STX, AddressingMode::ZeroPageY),
            0x8e => Instruction(Opcode::STX, AddressingMode::Absolute),
            
            0x84 => Instruction(Opcode::STY, AddressingMode::ZeroPage),
            0x94 => Instruction(Opcode::STY, AddressingMode::ZeroPageX),
            0x8c => Instruction(Opcode::STY, AddressingMode::Absolute),
            
            
            /* *************** set/clear flag ***************  */
            0x38 => Instruction(Opcode::SEC, AddressingMode::Implied),
            0xf8 => Instruction(Opcode::SED, AddressingMode::Implied),
            0x78 => Instruction(Opcode::SEI, AddressingMode::Implied),
            0x18 => Instruction(Opcode::CLC, AddressingMode::Implied),
            0xd8 => Instruction(Opcode::CLD, AddressingMode::Implied),
            0x58 => Instruction(Opcode::CLI, AddressingMode::Implied),
            0xb8 => Instruction(Opcode::CLV, AddressingMode::Implied),

            /* *************** compare ***************  */
            0xc9 => Instruction(Opcode::CMP, AddressingMode::Immediate),
            0xc5 => Instruction(Opcode::CMP, AddressingMode::ZeroPage),
            0xd5 => Instruction(Opcode::CMP, AddressingMode::ZeroPageX),
            0xcd => Instruction(Opcode::CMP, AddressingMode::Absolute),
            0xdd => Instruction(Opcode::CMP, AddressingMode::AbsoluteX),
            0xd9 => Instruction(Opcode::CMP, AddressingMode::AbsoluteY),
            0xc1 => Instruction(Opcode::CMP, AddressingMode::IndirectX),
            0xd1 => Instruction(Opcode::CMP, AddressingMode::IndirectY),

            0xe0 => Instruction(Opcode::CPX, AddressingMode::Immediate),
            0xe4 => Instruction(Opcode::CPX, AddressingMode::ZeroPage),
            0xec => Instruction(Opcode::CPX, AddressingMode::Absolute),

            0xc0 => Instruction(Opcode::CPY, AddressingMode::Immediate),
            0xc4 => Instruction(Opcode::CPY, AddressingMode::ZeroPage),
            0xcc => Instruction(Opcode::CPY, AddressingMode::Absolute),

            /* *************** jump/return ***************  */
            0x4c => Instruction(Opcode::JMP, AddressingMode::Absolute),
            0x6c => Instruction(Opcode::JMP, AddressingMode::Indirect),

            0x20 => Instruction(Opcode::JSR, AddressingMode::Absolute),

            0x40 => Instruction(Opcode::RTI, AddressingMode::Implied),
            0x60 => Instruction(Opcode::RTS, AddressingMode::Implied),

            /* *************** branch ***************  */
            0x90 => Instruction(Opcode::BCC, AddressingMode::Relative),
            0xb0 => Instruction(Opcode::BCS, AddressingMode::Relative),
            0xf0 => Instruction(Opcode::BEQ, AddressingMode::Relative),
            0x30 => Instruction(Opcode::BMI, AddressingMode::Relative),
            0xd0 => Instruction(Opcode::BNE, AddressingMode::Relative),
            0x10 => Instruction(Opcode::BPL, AddressingMode::Relative),
            0x50 => Instruction(Opcode::BVC, AddressingMode::Relative),
            0x70 => Instruction(Opcode::BVS, AddressingMode::Relative),

            /* *************** push/pop ***************  */
            0x48 => Instruction(Opcode::PHA, AddressingMode::Implied),
            0x08 => Instruction(Opcode::PHP, AddressingMode::Implied),
            0x68 => Instruction(Opcode::PLA, AddressingMode::Implied),
            0x28 => Instruction(Opcode::PLP, AddressingMode::Implied),

            /* *************** transfer ***************  */
            0xaa => Instruction(Opcode::TAX, AddressingMode::Implied),
            0xa8 => Instruction(Opcode::TAY, AddressingMode::Implied),
            0xba => Instruction(Opcode::TSX, AddressingMode::Implied),
            0x8a => Instruction(Opcode::TXA, AddressingMode::Implied),
            0x9a => Instruction(Opcode::TXS, AddressingMode::Implied),
            0x98 => Instruction(Opcode::TYA, AddressingMode::Implied),

            /* *************** other ***************  */
            0x00 => Instruction(Opcode::BRK, AddressingMode::Implied),

            0x24 => Instruction(Opcode::BIT, AddressingMode::ZeroPage),
            0x2c => Instruction(Opcode::BIT, AddressingMode::Absolute),

            0xea => Instruction(Opcode::NOP, AddressingMode::Absolute),

            _ => panic!("Invalid inst_code:{:08x}", inst_code),
        }
    }
}

impl Cpu {
    /// PCから1byteフェッチします
    /// フェッチした後、PCを一つ進めます
    pub fn fetch_u8(&mut self, system: &mut System) -> u8 {
        let data = system.read_u8(self.pc, false);
        self.pc = self.pc + 1;
        data
    }
    /// PCから2byteフェッチします
    /// フェッチした後、PCを一つ進めます
    pub fn fetch_u16(&mut self, system: &mut System) -> u16 {
        let lower = self.fetch_u8(system);
        let upper = self.fetch_u8(system);
        let data = u16::from(lower) | (u16::from(upper) << 8);
        data
    }
    /// operandをフェッチします。AddressingモードによってはPCも進みます
    /// 実装するときは命令直後のオペランドを読み取るときはCpu::fetch, それ以外はSystem::read
    pub fn fetch_operand(&mut self, system: &mut System, mode: AddressingMode) -> Operand {
        match mode {
            AddressingMode::Implied     => Operand(0, 0),
            AddressingMode::Accumulator => Operand(0, 0),
            AddressingMode::Immediate   => Operand(u16::from(self.fetch_u8(system)) , 1),
            AddressingMode::Absolute    => Operand(self.fetch_u16(system), 3),
            AddressingMode::ZeroPage    => Operand(u16::from(self.fetch_u8(system)) , 2),
            AddressingMode::ZeroPageX   => Operand(u16::from(self.fetch_u8(system).wrapping_add(self.x)), 3),
            AddressingMode::ZeroPageY   => Operand(u16::from(self.fetch_u8(system).wrapping_add(self.y)), 3),
            AddressingMode::AbsoluteX   => {
                let data = self.fetch_u16(system).wrapping_add(u16::from(self.x));
                let additional_cyc = if (data & 0xff00u16) != (data.wrapping_add(u16::from(self.x)) & 0xff00u16) { 1 } else { 0 };
                Operand(data, 3 + additional_cyc)
            },
            AddressingMode::AbsoluteY   => {
                let data = self.fetch_u16(system).wrapping_add(u16::from(self.y));
                let additional_cyc = if (data & 0xff00u16) != (data.wrapping_add(u16::from(self.y)) & 0xff00u16) { 1 } else { 0 };
                Operand(data, 3 + additional_cyc)
            },
            AddressingMode::Relative    => {
                let src_addr = self.fetch_u8(system);
                let signed_data = ((src_addr as i8) as i32) + (self.pc as i32); // 符号拡張して計算する
                debug_assert!(signed_data >= 0);
                debug_assert!(signed_data < 0x10000);

                let data = signed_data as u16 + 1; // 戻り先は指定+1にあるっぽい
                let additional_cyc = if (data & 0xff00u16) != (self.pc & 0xff00u16) { 1 } else { 0 };

                Operand(data, 1 + additional_cyc)
            },
            AddressingMode::Indirect    => {
                let src_addr_lower = self.fetch_u8(system);
                let src_addr_upper = self.fetch_u8(system);

                let dst_addr_lower = u16::from(src_addr_lower) | (u16::from(src_addr_upper) << 8); // operandそのまま
                let dst_addr_upper = u16::from(src_addr_lower.wrapping_add(1)) | (u16::from(src_addr_upper) << 8); // operandのlowerに+1したもの

                let dst_data_lower = u16::from(system.read_u8(dst_addr_lower, false));
                let dst_data_upper = u16::from(system.read_u8(dst_addr_upper, false));

                let data = dst_data_lower | (dst_data_upper << 8);

                Operand(data, 4)
            },
            AddressingMode::IndirectX   => {
                let src_addr = self.fetch_u8(system);
                let dst_addr = src_addr.wrapping_add(self.x);

                let data_lower = u16::from(system.read_u8(u16::from(dst_addr), false));
                let data_upper = u16::from(system.read_u8(u16::from(dst_addr.wrapping_add(1)), false));

                let data = data_lower | (data_upper << 8);
                Operand(data, 5)
            },
            AddressingMode::IndirectY   => {
                let src_addr = self.fetch_u8(system);

                let data_lower = u16::from(system.read_u8(u16::from(src_addr), false));
                let data_upper = u16::from(system.read_u8(u16::from(src_addr.wrapping_add(1)), false));

                let base_data = data_lower | (data_upper << 8);
                let data = base_data + u16::from(self.y);
                let additional_cyc = if (base_data & 0xff00u16) != (data & 0xff00u16) { 1 } else { 0 };

                Operand(data, 4 + additional_cyc)
            },
        }
    }

    /// 命令を実行します
    /// ret: cycle数
    pub fn run(&mut self, system: &mut System, inst_code: u8) -> u8 {
        let Instruction(opcode, mode) = Instruction::from(inst_code);
        match opcode {
            ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            Opcode::ADC => {
                let Operand(data, cyc) = self.fetch_operand(system, mode);
                let arg = data as u8;

                let (data1, is_carry1) = self.a.overflowing_add(arg as u8);
                let (result, is_carry2) = data1.overflowing_add(if self.read_carry_flag() { 1 } else { 0 } );

                let is_carry    = is_carry1 || is_carry2;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_overflow = (!(self.a ^ arg) & (self.a ^ result) & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result;
                1 + cyc
            },
            // TODO: 他の命令もここに移植





            ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            _ => panic!("Invalid Opcode"),
        }
    }
}

/// Instruction Implementation
/// http://obelisk.me.uk/6502/reference.html
/// 戻り値: 条件分岐等で余計にかかるclock cycle
impl Cpu {
    /// add with carry
    pub fn inst_adc(&mut self, arg: u8) -> u8 {
        let (data1, is_carry1) = self.a.overflowing_add(arg);
        let (result, is_carry2) = data1.overflowing_add(if self.read_carry_flag() { 1 } else { 0 } );

        let is_carry    = is_carry1 || is_carry2;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;
        let is_overflow = (!(self.a ^ arg) & (self.a ^ result) & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.write_overflow_flag(is_overflow);
        self.a = result;
        0
    }
    /// logical and
    pub fn inst_and(&mut self, arg: u8) -> u8{
        let result = self.a & arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// arithmetic shift left(Accumulator)
    pub fn inst_asl_a(&mut self) -> u8 {
        let (result, is_carry) = self.a.overflowing_shl(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// arithmetic shift left
    pub fn inst_asl(&mut self, system: &mut System, dst_addr: u16, arg: u8) -> u8 {
        let (result, is_carry) = arg.overflowing_shl(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result, false);
        0
    }
    /// branch if carry clear
    pub fn inst_bcc(&mut self, dst_addr: u16) -> u8 {
        if !self.read_carry_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// branch if carry set
    pub fn inst_bcs(&mut self, dst_addr: u16) -> u8 {
        if self.read_carry_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// branch if equal
    pub fn inst_beq(&mut self, dst_addr: u16) -> u8 {
        if self.read_zero_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// bit test
    pub fn inst_bit(&mut self, arg: u8) -> u8 {
        let is_negative = (arg & 0x80) == 0x80;
        let is_overflow = (arg & 0x40) == 0x40;
        let is_zero     = is_negative && is_overflow;

        self.write_negative_flag(is_negative);
        self.write_zero_flag(is_zero);
        self.write_overflow_flag(is_overflow);
        0
    }
    /// branch if minus
    pub fn inst_bmi(&mut self, dst_addr: u16) -> u8 {
        if self.read_negative_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// branch if not equal
    pub fn inst_bne(&mut self, dst_addr: u16) -> u8{
        if !self.read_zero_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// branch if plus
    pub fn inst_bpl(&mut self, dst_addr: u16) -> u8 {
        if !self.read_negative_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// force interrupt
    pub fn inst_brk(&mut self, system: &mut System) -> u8 {
        self.write_break_flag(true);
        self.interrupt(system, Interrupt::BRK);
        0
    }
    /// branch if overflow clear
    pub fn inst_bvc(&mut self, dst_addr: u16) -> u8 {
        if !self.read_overflow_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// branch if overflow set
    pub fn inst_bvs(&mut self, dst_addr: u16) -> u8 {
        if self.read_overflow_flag() {
            self.pc = dst_addr;
            1
        } else {
            0
        }
    }
    /// clear carry flag
    pub fn inst_clc(&mut self) -> u8 {
        self.write_carry_flag(false);
        0
    }
    /// clear decimal mode
    pub fn inst_cld(&mut self) -> u8 {
        self.write_decimal_flag(false);
        0
    }
    /// clear interrupt disable
    pub fn inst_cli(&mut self) -> u8 {
        self.write_interrupt_flag(false);
        0
    }
    /// clear overflow flag
    pub fn inst_clv(&mut self) -> u8 {
        self.write_overflow_flag(false);
        0
    }
    /// compare
    pub fn inst_cmp(&mut self, arg: u8) -> u8 {
        let (result, is_carry) = self.a.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        0
    }
    /// compare x register
    pub fn inst_cpx(&mut self, arg: u8) -> u8 {
        let (result, is_carry) = self.x.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        0
    }
    /// compare y register
    pub fn inst_cpy(&mut self, arg: u8) -> u8 {
        let (result, is_carry) = self.y.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        0
    }
    /// decrement memory
    pub fn inst_dec(&mut self, system: &mut System, dst_addr: u16, arg: u8) -> u8 {
        let result = arg.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result, false);
        0
    }
    /// decrement x register
    pub fn inst_dex(&mut self) -> u8 {
        let result = self.x.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
        0
    }
    /// decrement y register
    pub fn inst_dey(&mut self) -> u8 {
        let result = self.y.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = result;
        0
    }
    /// exclusive or
    pub fn inst_eor(&mut self, arg: u8) -> u8 {
        let result =self.a ^ arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// increment memory
    pub fn inst_inc(&mut self, system: &mut System, dst_addr: u16, arg: u8) -> u8 {
        let result = arg.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result, false);
        0
    }
    /// increment x register
    pub fn inst_inx(&mut self) -> u8 {
        let result = self.x.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
        0
    }
    /// increment y register
    pub fn inst_iny(&mut self) -> u8 {
        let result = self.y.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = result;
        0
    }
    /// jump
    /// `dst_addr` - Addressing Absolute/Indirectで指定されたJump先Address
    pub fn inst_jmp(&mut self, dst_addr: u16) -> u8 {
        self.pc = dst_addr;
        0
    }
    /// jump to subroutine
    /// `dst_addr` - Addressing Absoluteで指定されたJump先Address
    /// `opcode_addr` - JSR命令が格納されていたアドレス
    pub fn inst_jsr(&mut self, system: &mut System, dst_addr: u16, opcode_addr: u16) -> u8 {
        let ret_addr = opcode_addr + 2;
        // pushはUpper, Lower
        self.stack_push(system, (ret_addr >>   8) as u8);
        self.stack_push(system, (ret_addr & 0xff) as u8);
        self.pc = dst_addr;
        0
    }
    /// load accumulator
    pub fn inst_lda(&mut self, arg: u8) -> u8 {
        let is_zero     = arg == 0;
        let is_negative = (arg & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = arg;
        0
    }
    /// load x register
    pub fn inst_ldx(&mut self, arg: u8) -> u8 {
        let is_zero     = arg == 0;
        let is_negative = (arg & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = arg;
        0
    }
    /// load y register
    pub fn inst_ldy(&mut self, arg: u8) -> u8 {
        let is_zero     = arg == 0;
        let is_negative = (arg & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = arg;
        0
    }
    /// logical shift right(Accumulator)
    pub fn inst_lsr_a(&mut self) -> u8 {
        let result = self.a.wrapping_shr(1);

        let is_carry    = (self.a & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// logical shift right
    pub fn inst_lsr(&mut self, system: &mut System, dst_addr: u16, arg: u8) -> u8 {
        let result = arg.wrapping_shr(1);

        let is_carry    = (arg    & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, arg, false);
        0
    }
    // no operation
    pub fn inst_nop(&mut self) -> u8 {
        0
    }
    /// logical inclusive or
    pub fn inst_ora(&mut self, arg: u8) -> u8 {
        let result = self.a | arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// push accumulator
    pub fn inst_pha(&mut self, system: &mut System) -> u8 {
        self.stack_push(system, self.a);
        0
    }
    /// push processor status
    pub fn inst_php(&mut self, system: &mut System) -> u8 {
        self.stack_push(system, self.p);
        0
    }
    /// pull accumulator
    pub fn inst_pla(&mut self, system: &mut System) -> u8 {
        let result = self.stack_pop(system);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// pull processor status
    pub fn inst_plp(&mut self, system: &mut System) -> u8 {
        self.p = self.stack_pop(system);
        0
    }
    /// rorate left(Accumulator)
    pub fn inst_rol_a(&mut self) -> u8 {
        let result = self.a.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

        let is_carry    = (self.a & 0x80) == 0x80;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// rorate left
    pub fn inst_rol(&mut self, system: &mut System, dst_addr: u16, arg: u8) -> u8 {
        let result = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

        let is_carry    = (arg    & 0x80) == 0x80;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result, false);
        0
    }
    /// rorate right(Accumulator)
    pub fn inst_ror_a(&mut self) -> u8 {
        let result = self.a.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

        let is_carry    = (self.a & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
        0
    }
    /// rorate right
    pub fn inst_ror(&mut self, system: &mut System, dst_addr: u16, arg: u8) -> u8 {
        let result = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

        let is_carry    = (arg & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result, false);
        0
    }
    /// return from interuppt
    pub fn inst_rti(&mut self, system: &mut System) -> u8 {
        self.p = self.stack_pop(system);
        let pc_lower = self.stack_pop(system);
        let pc_upper = self.stack_pop(system);
        self.pc = ((pc_upper as u16) << 8) | (pc_lower as u16);
        0
    }
    /// return from subroutine
    pub fn inst_rts(&mut self, system: &mut System) -> u8 {
        let pc_lower = self.stack_pop(system);
        let pc_upper = self.stack_pop(system);
        self.pc = (((pc_upper as u16) << 8) | (pc_lower as u16)) + 1;
        0
    }
    /// subtract with carry
    /// A = A-arg-(1-Carry)
    pub fn inst_sbc(&mut self, arg: u8) -> u8 {
        let (data1, is_carry1) = self.a.overflowing_sub(arg);
        let (result, is_carry2) = data1.overflowing_sub(if self.read_carry_flag() { 0 } else { 1 } );

        let is_carry    = !(is_carry1 || is_carry2); // アンダーフローが発生したら0
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;
        let is_overflow = (!(self.a ^ arg) & (self.a ^ result) & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.write_overflow_flag(is_overflow);
        self.a = result;
        0
    }
    /// set carry flag
    pub fn inst_sec(&mut self) -> u8 {
        self.write_carry_flag(true);
        0
    }
    /// set decimal mode
    pub fn inst_sed(&mut self) -> u8 {
        self.write_decimal_flag(true);
        0
    }
    /// set interrupt disable
    pub fn inst_sei(&mut self) -> u8 {
        self.write_interrupt_flag(true);
        0
    }
    /// store accumulator
    pub fn inst_sta(&self, system: &mut System, dst_addr: u16) -> u8 {
        system.write_u8(dst_addr, self.a, false);
        0
    }
    /// store x register
    pub fn inst_stx(&self, system: &mut System, dst_addr: u16) -> u8 {
        system.write_u8(dst_addr, self.x, false);
        0
    }
    /// store y register
    pub fn inst_sty(&self, system: &mut System, dst_addr: u16) -> u8 {
        system.write_u8(dst_addr, self.y, false);
        0
    }
    /// transfer accumulator to x
    pub fn inst_tax(&mut self) -> u8 {
        let is_zero     = self.a == 0;
        let is_negative = (self.a & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = self.a;
        0
    }
    /// transfer accumulator to y
    pub fn inst_tay(&mut self) -> u8 {
        let is_zero     = self.a == 0;
        let is_negative = (self.a & 0x80) == 0x80;
        
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = self.a;
        0
    }
    /// transfer stack pointer to x
    pub fn inst_tsx(&mut self) -> u8 {
        let result = (self.sp & 0xff) as u8;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;
        
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
        0
    }
    /// transfer x to accumulator
    pub fn inst_txa(&mut self) -> u8 {
        let is_zero     = self.x == 0;
        let is_negative = (self.x & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = self.x;
        0
    }
    /// transfer x to stack pointer
    pub fn inst_txs(&mut self) -> u8 {
        // spの上位バイトは0x01固定
        // txsはstatus書き換えなし
        self.sp = (self.x as u16) | 0x0100u16; 
        0
    }
    /// transfer y to accumulator
    pub fn inst_tya(&mut self) -> u8 {
        let is_zero     = self.y == 0;
        let is_negative = (self.y & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = self.y;
        0
    }
}
