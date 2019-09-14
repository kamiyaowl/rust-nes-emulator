use super::cpu::*;
use super::interface::SystemBus;
use super::ppu::Ppu; // TODO: 削除
use super::system::System;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Opcode {
    // binary op
    ADC,
    SBC,
    AND,
    EOR,
    ORA,
    // shift/rotate
    ASL,
    LSR,
    ROL,
    ROR,
    // inc/dec
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    // load/store
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    // set/clear flag
    SEC,
    SED,
    SEI,
    CLC,
    CLD,
    CLI,
    CLV,
    // compare
    CMP,
    CPX,
    CPY,
    // jump return
    JMP,
    JSR,
    RTI,
    RTS,
    // branch
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    // push/pop
    PHA,
    PHP,
    PLA,
    PLP,
    // transfer
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    // other
    BRK,
    BIT,
    NOP,
    // unofficial1
    // https://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes
    ALR,
    ANC,
    ARR,
    AXS,
    LAX,
    SAX,
    DCP,
    ISC,
    RLA,
    RRA,
    SLO,
    SRE,
    SKB,
    IGN,
    // unofficial2
    //ADC, SBC, NOP,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum AddressingMode {
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
struct Operand(u16, u8);

#[derive(Copy, Clone, Debug)]
struct Instruction(Opcode, AddressingMode);

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
            0xd0 => Instruction(Opcode::BNE, AddressingMode::Relative),
            0x30 => Instruction(Opcode::BMI, AddressingMode::Relative),
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

            0xea => Instruction(Opcode::NOP, AddressingMode::Implied),

            /* *************** unofficial1 ***************  */
            0x4b => Instruction(Opcode::ALR, AddressingMode::Immediate),
            0x0b => Instruction(Opcode::ANC, AddressingMode::Immediate),
            0x6b => Instruction(Opcode::ARR, AddressingMode::Immediate),
            0xcb => Instruction(Opcode::AXS, AddressingMode::Immediate),

            0xa3 => Instruction(Opcode::LAX, AddressingMode::IndirectX),
            0xa7 => Instruction(Opcode::LAX, AddressingMode::ZeroPage),
            0xaf => Instruction(Opcode::LAX, AddressingMode::Absolute),
            0xb3 => Instruction(Opcode::LAX, AddressingMode::IndirectY),
            0xb7 => Instruction(Opcode::LAX, AddressingMode::ZeroPageY),
            0xbf => Instruction(Opcode::LAX, AddressingMode::AbsoluteY),

            0x83 => Instruction(Opcode::SAX, AddressingMode::IndirectX),
            0x87 => Instruction(Opcode::SAX, AddressingMode::ZeroPage),
            0x8f => Instruction(Opcode::SAX, AddressingMode::Absolute),
            0x97 => Instruction(Opcode::SAX, AddressingMode::ZeroPageY),

            0xc3 => Instruction(Opcode::DCP, AddressingMode::IndirectX),
            0xc7 => Instruction(Opcode::DCP, AddressingMode::ZeroPage),
            0xcf => Instruction(Opcode::DCP, AddressingMode::Absolute),
            0xd3 => Instruction(Opcode::DCP, AddressingMode::IndirectY),
            0xd7 => Instruction(Opcode::DCP, AddressingMode::ZeroPageX),
            0xdb => Instruction(Opcode::DCP, AddressingMode::AbsoluteY),
            0xdf => Instruction(Opcode::DCP, AddressingMode::AbsoluteX),

            0xe3 => Instruction(Opcode::ISC, AddressingMode::IndirectX),
            0xe7 => Instruction(Opcode::ISC, AddressingMode::ZeroPage),
            0xef => Instruction(Opcode::ISC, AddressingMode::Absolute),
            0xf3 => Instruction(Opcode::ISC, AddressingMode::IndirectY),
            0xf7 => Instruction(Opcode::ISC, AddressingMode::ZeroPageX),
            0xfb => Instruction(Opcode::ISC, AddressingMode::AbsoluteY),
            0xff => Instruction(Opcode::ISC, AddressingMode::AbsoluteX),

            0x23 => Instruction(Opcode::RLA, AddressingMode::IndirectX),
            0x27 => Instruction(Opcode::RLA, AddressingMode::ZeroPage),
            0x2f => Instruction(Opcode::RLA, AddressingMode::Absolute),
            0x33 => Instruction(Opcode::RLA, AddressingMode::IndirectY),
            0x37 => Instruction(Opcode::RLA, AddressingMode::ZeroPageX),
            0x3b => Instruction(Opcode::RLA, AddressingMode::AbsoluteY),
            0x3f => Instruction(Opcode::RLA, AddressingMode::AbsoluteX),

            0x63 => Instruction(Opcode::RRA, AddressingMode::IndirectX),
            0x67 => Instruction(Opcode::RRA, AddressingMode::ZeroPage),
            0x6f => Instruction(Opcode::RRA, AddressingMode::Absolute),
            0x73 => Instruction(Opcode::RRA, AddressingMode::IndirectY),
            0x77 => Instruction(Opcode::RRA, AddressingMode::ZeroPageX),
            0x7b => Instruction(Opcode::RRA, AddressingMode::AbsoluteY),
            0x7f => Instruction(Opcode::RRA, AddressingMode::AbsoluteX),

            0x03 => Instruction(Opcode::SLO, AddressingMode::IndirectX),
            0x07 => Instruction(Opcode::SLO, AddressingMode::ZeroPage),
            0x0f => Instruction(Opcode::SLO, AddressingMode::Absolute),
            0x13 => Instruction(Opcode::SLO, AddressingMode::IndirectY),
            0x17 => Instruction(Opcode::SLO, AddressingMode::ZeroPageX),
            0x1b => Instruction(Opcode::SLO, AddressingMode::AbsoluteY),
            0x1f => Instruction(Opcode::SLO, AddressingMode::AbsoluteX),

            0x43 => Instruction(Opcode::SRE, AddressingMode::IndirectX),
            0x47 => Instruction(Opcode::SRE, AddressingMode::ZeroPage),
            0x4f => Instruction(Opcode::SRE, AddressingMode::Absolute),
            0x53 => Instruction(Opcode::SRE, AddressingMode::IndirectY),
            0x57 => Instruction(Opcode::SRE, AddressingMode::ZeroPageX),
            0x5b => Instruction(Opcode::SRE, AddressingMode::AbsoluteY),
            0x5f => Instruction(Opcode::SRE, AddressingMode::AbsoluteX),

            0x80 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0x82 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0x89 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0xc2 => Instruction(Opcode::SKB, AddressingMode::Immediate),
            0xe2 => Instruction(Opcode::SKB, AddressingMode::Immediate),

            0x0c => Instruction(Opcode::IGN, AddressingMode::Absolute),

            0x1c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0x3c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0x5c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0x7c => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0xdc => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),
            0xfc => Instruction(Opcode::IGN, AddressingMode::AbsoluteX),

            0x04 => Instruction(Opcode::IGN, AddressingMode::ZeroPage),
            0x44 => Instruction(Opcode::IGN, AddressingMode::ZeroPage),
            0x64 => Instruction(Opcode::IGN, AddressingMode::ZeroPage),

            0x14 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0x34 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0x54 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0x74 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0xd4 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),
            0xf4 => Instruction(Opcode::IGN, AddressingMode::ZeroPageX),

            /* *************** unofficial2(既存の命令) ***************  */
            0xeb => Instruction(Opcode::SBC, AddressingMode::Immediate),

            0x1a => Instruction(Opcode::NOP, AddressingMode::Implied),
            0x3a => Instruction(Opcode::NOP, AddressingMode::Implied),
            0x5a => Instruction(Opcode::NOP, AddressingMode::Implied),
            0x7a => Instruction(Opcode::NOP, AddressingMode::Implied),
            0xda => Instruction(Opcode::NOP, AddressingMode::Implied),
            0xfa => Instruction(Opcode::NOP, AddressingMode::Implied),

            _ => panic!("Invalid inst_code:{:08x}", inst_code),
        }
    }
}

impl Cpu {
    /// PCから1byteフェッチします
    /// フェッチした後、PCを一つ進めます
    fn fetch_u8(&mut self, system: &mut System) -> u8 {
        let data = system.read_u8(self.pc, false);
        self.pc = self.pc + 1;
        data
    }
    /// PCから2byteフェッチします
    /// フェッチした後、PCを一つ進めます
    fn fetch_u16(&mut self, system: &mut System) -> u16 {
        let lower = self.fetch_u8(system);
        let upper = self.fetch_u8(system);
        let data = u16::from(lower) | (u16::from(upper) << 8);
        data
    }
    /// operandをフェッチします。AddressingモードによってはPCも進みます
    /// 実装するときは命令直後のオペランドを読み取るときはCpu::fetch, それ以外はSystem::read
    fn fetch_operand(&mut self, system: &mut System, mode: AddressingMode) -> Operand {
        match mode {
            AddressingMode::Implied => Operand(0, 0),
            AddressingMode::Accumulator => Operand(0, 1),
            AddressingMode::Immediate => Operand(u16::from(self.fetch_u8(system)), 1),
            AddressingMode::Absolute => Operand(self.fetch_u16(system), 3),
            AddressingMode::ZeroPage => Operand(u16::from(self.fetch_u8(system)), 2),
            AddressingMode::ZeroPageX => {
                Operand(u16::from(self.fetch_u8(system).wrapping_add(self.x)), 3)
            }
            AddressingMode::ZeroPageY => {
                Operand(u16::from(self.fetch_u8(system).wrapping_add(self.y)), 3)
            }
            AddressingMode::AbsoluteX => {
                let data = self.fetch_u16(system).wrapping_add(u16::from(self.x));
                let additional_cyc =
                    if (data & 0xff00u16) != (data.wrapping_add(u16::from(self.x)) & 0xff00u16) {
                        1
                    } else {
                        0
                    };
                Operand(data, 3 + additional_cyc)
            }
            AddressingMode::AbsoluteY => {
                let data = self.fetch_u16(system).wrapping_add(u16::from(self.y));
                let additional_cyc =
                    if (data & 0xff00u16) != (data.wrapping_add(u16::from(self.y)) & 0xff00u16) {
                        1
                    } else {
                        0
                    };
                Operand(data, 3 + additional_cyc)
            }
            AddressingMode::Relative => {
                let src_addr = self.fetch_u8(system);
                let signed_data = ((src_addr as i8) as i32) + (self.pc as i32); // 符号拡張して計算する
                debug_assert!(signed_data >= 0);
                debug_assert!(signed_data < 0x10000);

                let data = signed_data as u16;
                let additional_cyc = if (data & 0xff00u16) != (self.pc & 0xff00u16) {
                    1
                } else {
                    0
                };

                Operand(data, 1 + additional_cyc)
            }
            AddressingMode::Indirect => {
                let src_addr_lower = self.fetch_u8(system);
                let src_addr_upper = self.fetch_u8(system);

                let dst_addr_lower = u16::from(src_addr_lower) | (u16::from(src_addr_upper) << 8); // operandそのまま
                let dst_addr_upper =
                    u16::from(src_addr_lower.wrapping_add(1)) | (u16::from(src_addr_upper) << 8); // operandのlowerに+1したもの

                let dst_data_lower = u16::from(system.read_u8(dst_addr_lower, false));
                let dst_data_upper = u16::from(system.read_u8(dst_addr_upper, false));

                let data = dst_data_lower | (dst_data_upper << 8);

                Operand(data, 5)
            }
            AddressingMode::IndirectX => {
                let src_addr = self.fetch_u8(system);
                let dst_addr = src_addr.wrapping_add(self.x);

                let data_lower = u16::from(system.read_u8(u16::from(dst_addr), false));
                let data_upper =
                    u16::from(system.read_u8(u16::from(dst_addr.wrapping_add(1)), false));

                let data = data_lower | (data_upper << 8);
                Operand(data, 5)
            }
            AddressingMode::IndirectY => {
                let src_addr = self.fetch_u8(system);

                let data_lower = u16::from(system.read_u8(u16::from(src_addr), false));
                let data_upper =
                    u16::from(system.read_u8(u16::from(src_addr.wrapping_add(1)), false));

                let base_data = data_lower | (data_upper << 8);
                let data = base_data.wrapping_add(u16::from(self.y));
                let additional_cyc = if (base_data & 0xff00u16) != (data & 0xff00u16) {
                    1
                } else {
                    0
                };

                Operand(data, 4 + additional_cyc)
            }
        }
    }
    /// addressだけでなくデータまで一発で引きたい場合
    /// ret: (Operand(引いだ即値もしくはアドレス, clock数), データ)
    fn fetch_args(&mut self, system: &mut System, mode: AddressingMode) -> (Operand, u8) {
        match mode {
            // 使わないはず
            AddressingMode::Implied => (self.fetch_operand(system, mode), 0),
            // aレジスタの値を使う
            AddressingMode::Accumulator => (self.fetch_operand(system, mode), self.a),
            // 即値はopcodeの直後のデータ1byteをそのまま使う
            AddressingMode::Immediate => {
                let Operand(data, cyc) = self.fetch_operand(system, mode);
                debug_assert!(data < 0x100u16);
                (Operand(data, cyc), data as u8)
            }
            // 他は帰ってきたアドレスからデータを引きなおす。使わない場合もある
            _ => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                let data = system.read_u8(addr, false);
                (Operand(addr, cyc), data)
            }
        }
    }

    /// 命令を実行します
    /// ret: cycle数
    /// http://obelisk.me.uk/6502/reference.html
    // TODO: debug printようにppu借りる
    pub fn step(&mut self, system: &mut System, ppu: &Ppu) -> u8 {
        // 命令がおいてあるところのaddress
        let inst_pc = self.pc;
        let inst_code = self.fetch_u8(system);

        let Instruction(opcode, mode) = Instruction::from(inst_code);

        match opcode {
            /* *************** binary op ***************  */
            // 結果はaレジスタに格納するので、operandのアドレスは使わない
            Opcode::ADC => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let tmp = u16::from(self.a) + u16::from(arg) + (if self.read_carry_flag() { 1 } else { 0 } );
                let result = (tmp & 0xff) as u8;

                let is_carry    = tmp > 0x00ffu16;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_overflow = ((self.a ^ result) & (arg ^ result) & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result;
                1 + cyc
            },
            Opcode::SBC => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (data1, is_carry1) = self.a.overflowing_sub(arg);
                let (result, is_carry2) = data1.overflowing_sub(if self.read_carry_flag() { 0 } else { 1 } );

                let is_carry    = !(is_carry1 || is_carry2); // アンダーフローが発生したら0
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_overflow = (((self.a ^ arg) & 0x80) == 0x80) && (((self.a ^ result) & 0x80) == 0x80);

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result;
                1 + cyc
            },
            Opcode::AND => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a & arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result;
                1 + cyc
            },
            Opcode::EOR => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result =self.a ^ arg;

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result;
                1 + cyc
            },
            Opcode::ORA => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a | arg;

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result;
                1 + cyc
            },
            /* *************** shift/rotate op ***************  */
            // aレジスタを操作する場合があるので注意
            Opcode::ASL => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shl(1);

                let is_carry    = (arg    & 0x80) == 0x80; // shift前データでわかるよね
                let is_zero     =  result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else {
                    // 計算結果を元いたアドレスに書き戻す
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            Opcode::LSR => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shr(1);

                let is_carry    = (arg    & 0x01) == 0x01;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else {
                    // 計算結果を元いたアドレスに書き戻す
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            Opcode::ROL => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

                let is_carry    = (arg    & 0x80) == 0x80;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else {
                    // 計算結果を元いたアドレスに書き戻す
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            Opcode::ROR => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

                let is_carry    = (arg & 0x01) == 0x01;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + cyc
                } else {
                    // 計算結果を元いたアドレスに書き戻す
                    system.write_u8(addr, result, false);
                    3 + cyc
                }
            },
            /* *************** inc/dec op ***************  */
            // accumulatorは使わない, x,yレジスタを使うバージョンはImplied
            Opcode::INC => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_add(1);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                system.write_u8(addr, result, false);
                3 + cyc
            },
            Opcode::INX => {
                let result = self.x.wrapping_add(1);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = result;
                2
            },
            Opcode::INY => {
                let result = self.y.wrapping_add(1);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.y = result;
                2
            },
            Opcode::DEC => {
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                let result = arg.wrapping_sub(1);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                system.write_u8(addr, result, false);
                3 + cyc
            },
            Opcode::DEX => {
                let result = self.x.wrapping_sub(1);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = result;
                2
            },
            Opcode::DEY => {
                let result = self.y.wrapping_sub(1);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.y = result;
                2
            },

            /* *************** load/store op ***************  */
            // Accumualtorはなし
            // store系はargはいらない, Immediateなし
            Opcode::LDA => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let is_zero     = arg == 0;
                let is_negative = (arg & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = arg;
                1 + cyc
            },
            Opcode::LDX => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let is_zero     = arg == 0;
                let is_negative = (arg & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = arg;
                1 + cyc
            },
            Opcode::LDY => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let is_zero     = arg == 0;
                let is_negative = (arg & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.y = arg;
                1 + cyc
            },
            Opcode::STA => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);

                system.write_u8(addr, self.a, false);
                1 + cyc
            },
            Opcode::STX => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);

                system.write_u8(addr, self.x, false);
                1 + cyc
            },
            Opcode::STY => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);

                system.write_u8(addr, self.y, false);
                1 + cyc
            },

            /* *************** set/clear flag ***************  */
            // すべてImplied
            Opcode::SEC => {
                self.write_carry_flag(true);
                2
            },
            Opcode::SED => {
                self.write_decimal_flag(true);
                2
            },
            Opcode::SEI => {
                self.write_interrupt_flag(true);
                2
            },
            Opcode::CLC => {
                self.write_carry_flag(false);
                2
            },
            Opcode::CLD => {
                self.write_decimal_flag(false);
                2
            },
            Opcode::CLI => {
                self.write_interrupt_flag(false);
                2
            },
            Opcode::CLV => {
                self.write_overflow_flag(false);
                2
            },

            /* *************** compare ***************  */
            // Accumulatorなし
            Opcode::CMP => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (result, _) = self.a.overflowing_sub(arg);

                let is_carry    = self.a >= arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                1 + cyc
            },
            Opcode::CPX => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (result, _) = self.x.overflowing_sub(arg);

                let is_carry    = self.x >= arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                1 + cyc
            },
            Opcode::CPY => {
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let (result, _) = self.y.overflowing_sub(arg);

                let is_carry    = self.y >= arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                1 + cyc
            },

            /* *************** jump/return ***************  */
            // JMP: Absolute or Indirect, JSR: Absolute, RTI,RTS: Implied
            Opcode::JMP => {
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                self.pc = addr;
                cyc
            },
            Opcode::JSR => {
                let Operand(addr, _) = self.fetch_operand(system, mode);
                // opcodeがあったアドレスを取得する(opcode, operand fetchで3進んでる)
                let opcode_addr = inst_pc;

                // pushはUpper, Lower
                let ret_addr = opcode_addr + 2;
                self.stack_push(system, (ret_addr >>   8) as u8);
                self.stack_push(system, (ret_addr & 0xff) as u8);
                self.pc = addr;
                6
            },
            Opcode::RTI => {
                self.p = self.stack_pop(system);
                let pc_lower = self.stack_pop(system);
                let pc_upper = self.stack_pop(system);
                self.pc = ((pc_upper as u16) << 8) | (pc_lower as u16);
                6
            },
            Opcode::RTS => {
                let pc_lower = self.stack_pop(system);
                let pc_upper = self.stack_pop(system);
                self.pc = (((pc_upper as u16) << 8) | (pc_lower as u16)) + 1;
                6
            },

            /* *************** branch ***************  */
            // Relativeのみ
            Opcode::BCC => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_carry_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BCS => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_carry_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BEQ => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_zero_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BNE => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_zero_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BMI => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_negative_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BPL => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_negative_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BVC => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if !self.read_overflow_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },
            Opcode::BVS => {
                debug_assert!(mode == AddressingMode::Relative);
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                if self.read_overflow_flag() {
                    self.pc = addr;
                    1 + cyc + 1
                } else {
                    1 + cyc
                }
            },

            /* *************** push/pop ***************  */
            // Impliedのみ
            Opcode::PHA => {
                self.stack_push(system, self.a);
                3
            },
            Opcode::PHP => {
                self.stack_push(system, self.p);
                3

            },
            Opcode::PLA => {
                let result = self.stack_pop(system);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result;
                4
            },
            Opcode::PLP => {
                self.p = self.stack_pop(system);
                4
            },

            /* *************** transfer ***************  */
            // Impliedのみ
            Opcode::TAX => {
                let is_zero     = self.a == 0;
                let is_negative = (self.a & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = self.a;
                2
            },
            Opcode::TAY => {
                let is_zero     = self.a == 0;
                let is_negative = (self.a & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.y = self.a;
                2
            },
            Opcode::TSX => {
                let result = (self.sp & 0xff) as u8;

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = result;
                2
            },
            Opcode::TXA => {
                let is_zero     = self.x == 0;
                let is_negative = (self.x & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = self.x;
                2
            },
            Opcode::TXS => {
                // spの上位バイトは0x01固定
                // txsはstatus書き換えなし
                self.sp = (self.x as u16) | 0x0100u16;
                2
            },
            Opcode::TYA => {
                let is_zero     = self.y == 0;
                let is_negative = (self.y & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = self.y;
                2
            },

            /* *************** other ***************  */
            Opcode::BRK => {
                // Implied
                self.write_break_flag(true);
                self.interrupt(system, Interrupt::BRK);
                7
            },
            Opcode::BIT => {
                // ZeroPage or Absolute
                // 非破壊読み出しが必要, fetch_args使わずに自分で読むか...
                let Operand(addr, cyc) = self.fetch_operand(system, mode);
                let arg = system.read_u8(addr, true); // 非破壊読み出し

                let is_negative = (arg    & 0x80) == 0x80;
                let is_overflow = (arg    & 0x40) == 0x40;
                let is_zero     = (self.a & arg ) == 0x00;

                self.write_negative_flag(is_negative);
                self.write_zero_flag(is_zero);
                self.write_overflow_flag(is_overflow);
                2 + cyc
            },
            Opcode::NOP => {
                //なにもしない、Implied
                2
            },
            /* *************** unofficial1 ***************  */
            Opcode::ALR => {
                // Immediateのみ、(A & #Imm) >> 1
                debug_assert!(mode == AddressingMode::Immediate);
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let src = self.a & arg;
                let result = src.wrapping_shr(1);

                let is_carry    = (src    & 0x01) == 0x01;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                self.a = result;
                1 + cyc
            },
            Opcode::ANC => {
                // Immediateのみ、A=A & #IMM, Carryは前回状態のNegativeをコピー
                debug_assert!(mode == AddressingMode::Immediate);
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let result = self.a & arg;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_carry    = self.read_negative_flag();

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_carry_flag(is_carry);
                self.a = result;
                1 + cyc
            },
            Opcode::ARR => {
                // Immediateのみ、Carry=bit6, V=bit6 xor bit5
                debug_assert!(mode == AddressingMode::Immediate);
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let src = self.a & arg;
                let result = src.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_carry    = (result & 0x40) == 0x40;
                let is_overflow = ((result & 0x40) ^ ((result & 0x20) << 1)) == 0x40;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_carry_flag(is_carry);
                self.write_overflow_flag(is_overflow);

                self.a = result;
                1 + cyc
            },
            Opcode::AXS => {
                // Immediateのみ、X = (A & X) - #IMM, NZCを更新
                // without borrowとのことなので、減算時cフラグも無視
                debug_assert!(mode == AddressingMode::Immediate);
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let src = self.a & arg;

                let (result, is_carry) = self.a.overflowing_sub(src);

                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.x = result;
                1 + cyc
            },
            Opcode::LAX => {
                // A = X = argsっぽい
                let (Operand(_, cyc), arg) = self.fetch_args(system, mode);

                let is_zero     = arg == 0;
                let is_negative = (arg & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = arg;
                self.x = arg;
                1 + cyc
            },
            Opcode::SAX => {
                // memory = A & X, flag操作はなし
                let (Operand(addr, cyc), _arg) = self.fetch_args(system, mode);

                let result = self.a & self.x;

                system.write_u8(addr, result, false);
                1 + cyc
            },
            Opcode::DCP => {
                // DEC->CMPっぽい
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                // DEC
                let dec_result = arg.wrapping_sub(1);
                system.write_u8(addr, dec_result, false);

                // CMP
                let result = self.a.wrapping_sub(dec_result);

                let is_carry    = self.a >= dec_result;
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                3 + cyc
            },
            Opcode::ISC => {
                // INC->SBC
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                // INC
                let inc_result = arg.wrapping_add(1);
                system.write_u8(addr, inc_result, false);

                // SBC
                let (data1, is_carry1) = self.a.overflowing_sub(inc_result);
                let (result, is_carry2) = data1.overflowing_sub(if self.read_carry_flag() { 0 } else { 1 } );

                let is_carry    = !(is_carry1 || is_carry2); // アンダーフローが発生したら0
                let is_zero     = result == 0;
                let is_negative = (result & 0x80) == 0x80;
                let is_overflow = (((self.a ^ inc_result) & 0x80) == 0x80) && (((self.a ^ result) & 0x80) == 0x80);

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result;
                1 + cyc
            },
            Opcode::RLA => {
                // ROL -> AND
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                // ROL
                let result_rol = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

                let is_carry    = (arg & 0x80) == 0x80;
                self.write_carry_flag(is_carry);

                system.write_u8(addr, result_rol, false);

                // AND
                let result_and = self.a & result_rol;

                let is_zero     = result_and == 0;
                let is_negative = (result_and & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);

                self.a = result_and;

                3 + cyc
            },
            Opcode::RRA => {
                // ROR -> ADC
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                // ROR
                let result_ror = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

                let is_carry_ror    = (arg & 0x01) == 0x01;
                self.write_carry_flag(is_carry_ror);

                system.write_u8(addr, result_ror, false);

                // ADC
                let tmp = u16::from(self.a) + u16::from(result_ror) + (if self.read_carry_flag() { 1 } else { 0 } );
                let result_adc = (tmp & 0xff) as u8;

                let is_carry    = tmp > 0x00ffu16;
                let is_zero     = result_adc == 0;
                let is_negative = (result_adc & 0x80) == 0x80;
                let is_overflow = ((self.a ^ result_adc) & (result_ror ^ result_adc) & 0x80) == 0x80;

                self.write_carry_flag(is_carry);
                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.write_overflow_flag(is_overflow);
                self.a = result_adc;

                3 + cyc
            },
            Opcode::SLO => {
                // ASL -> ORA
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                // ASL
                let result_asl = arg.wrapping_shl(1);

                let is_carry    = (arg & 0x80) == 0x80; // shift前データでわかるよね
                self.write_carry_flag(is_carry);

                system.write_u8(addr, result_asl, false);

                // ORA
                let result_ora = self.a | result_asl;

                let is_zero     = result_ora == 0;
                let is_negative = (result_ora & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result_ora;

                3 + cyc
            },
            Opcode::SRE => {
                // LSR -> EOR
                let (Operand(addr, cyc), arg) = self.fetch_args(system, mode);

                // LSR
                let result_lsr = arg.wrapping_shr(1);

                let is_carry    = (arg & 0x01) == 0x01;
                self.write_carry_flag(is_carry);

                system.write_u8(addr, result_lsr, false);

                // EOR
                let result_eor = self.a ^ result_lsr;

                let is_zero     = result_eor == 0;
                let is_negative = (result_eor & 0x80) == 0x80;

                self.write_zero_flag(is_zero);
                self.write_negative_flag(is_negative);
                self.a = result_eor;

                3 + cyc
            },
            Opcode::SKB => {
                // Immediateをフェッチするけど、なにもしない
                debug_assert!(mode == AddressingMode::Immediate);
                let (Operand(_addr, cyc), _arg) = self.fetch_args(system, mode);

                1 + cyc
            },
            Opcode::IGN => {
                // フェッチするけど、なにもしない
                let (Operand(_addr, cyc), _arg) = self.fetch_args(system, mode);

                1 + cyc
            },
        }
    }
}
