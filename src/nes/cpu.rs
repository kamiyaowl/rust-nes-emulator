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
        self.sp = 0x01fd;
        self.p  = 0x34;
    }
    fn store(&self, read_callback: fn(usize, u8)) {
        // レジスタダンプを連番で取得する(little endian)
        read_callback( 0, self.a);
        read_callback( 1, self.x);
        read_callback( 2, self.y);
        read_callback( 3, (self.pc & 0xff) as u8);
        read_callback( 4, ((self.pc >> 8) & 0xff) as u8);
        read_callback( 5, (self.sp & 0xff) as u8);
        read_callback( 6, ((self.sp >> 8) & 0xff) as u8);
        read_callback( 7, self.p);
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
    pub fn do_interrupt(&mut self, system: &mut System, irq_type: Interrupt) {
        let is_nested_interrupt = self.read_interrupt_flag();
        // RESET, NMI以外は多重割り込みを許容しない
        if is_nested_interrupt && (irq_type == Interrupt::IRQ) || (irq_type == Interrupt::BRK) {
            return;
        }
        // 割り込み種類別の処理
        match irq_type {
            Interrupt::NMI   => {
                self.write_break_flag(false);
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            },
            Interrupt::RESET => {
                self.write_interrupt_flag(true);
            },
            Interrupt::IRQ   => {
                self.write_break_flag(false);
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);
            },
            Interrupt::BRK   => {
                self.write_break_flag(true);
                self.pc = self.pc + 1;
                // PCのUpper, Lower, Status RegisterをStackに格納する
                self.stack_push(system, (self.pc >> 8) as u8);
                self.stack_push(system, (self.pc & 0xff) as u8);
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
        system.read_u8(self.sp as usize)
    }

}
/// Instruction Implementation
/// http://obelisk.me.uk/6502/reference.html
impl Cpu {
    /// add with carry
    fn inst_adc(&mut self, arg: u8) {
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
    }
    /// logical and
    fn inst_and(&mut self, arg: u8) {
        let result = self.a & arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// arithmetic shift left(Accumulator)
    fn inst_asl_a(&mut self) {
        let (result, is_carry) = self.a.overflowing_shl(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// arithmetic shift left
    fn inst_asl(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let (result, is_carry) = arg.overflowing_shl(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr as usize, result);
    }
    /// branch if carry clear
    fn inst_bcc(&mut self, arg: u8) {
        if !self.read_carry_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if carry set
    fn inst_bcs(&mut self, arg: u8) {
        if self.read_carry_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if equal
    fn inst_beq(&mut self, arg: u8) {
        if self.read_zero_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// bit test
    fn inst_bit(&mut self, arg: u8) {
        let is_negative = (arg & 0x80) == 0x80;
        let is_overflow = (arg & 0x40) == 0x40;
        let is_zero     = is_negative && is_overflow;

        self.write_negative_flag(is_negative);
        self.write_zero_flag(is_zero);
        self.write_overflow_flag(is_overflow);
    }
    /// branch if minus
    fn inst_bmi(&mut self, arg: u8) {
        if self.read_negative_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if not equal
    fn inst_bne(&mut self, arg: u8) {
        if !self.read_zero_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if plus
    fn inst_bpl(&mut self, arg: u8) {
        if !self.read_negative_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// force interrupt
    fn inst_brk(&mut self, system: &mut System) {
        self.write_break_flag(true);
        self.do_interrupt(system, Interrupt::BRK);
    }
    /// branch if overflow clear
    fn inst_bvc(&mut self, arg: u8) {
        if !self.read_overflow_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if overflow set
    fn inst_bvs(&mut self, arg: u8) {
        if self.read_overflow_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// clear carry flag
    fn inst_clc(&mut self) {
        self.write_carry_flag(false);
    }
    /// clear decimal mode
    fn inst_cld(&mut self) {
        self.write_decimal_flag(false);
    }
    /// clear interrupt disable
    fn inst_cli(&mut self) {
        self.write_interrupt_flag(false);
    }
    /// clear overflow flag
    fn inst_clv(&mut self) {
        self.write_overflow_flag(false);
    }
    /// compare
    fn inst_cmp(&mut self, arg: u8) {
        let (result, is_carry) = self.a.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
    }
    /// compare x register
    fn inst_cpx(&mut self, arg: u8) {
        let (result, is_carry) = self.x.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
    }
    /// compare y register
    fn inst_cpy(&mut self, arg: u8) {
        let (result, is_carry) = self.y.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
    }
    /// decrement memory
    fn inst_dec(&mut self, arg: u8) -> u8 {
        let result = arg.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        result
    }
    /// decrement x register
    fn inst_dex(&mut self, arg: u8) {
        let result = self.x.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
    }
    /// decrement y register
    fn inst_dey(&mut self, arg: u8) {
        let result = self.y.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = result;
    }
    /// exclusive or
    fn inst_eor(&mut self, arg: u8) {
        let result =self.a ^ arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// increment memory
    fn inst_inc(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr as usize, result);
    }
    /// increment x register
    fn inst_inx(&mut self, arg: u8) {
        let result = self.x.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
    }
    /// increment y register
    fn inst_iny(&mut self, arg: u8) {
        let result = self.y.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = result;
    }
    /// jump
    /// `dst_addr` - Addressing Absolute/Indirectで指定されたJump先Address
    fn inst_jmp(&mut self, dst_addr: u16) {
        self.pc = dst_addr;
    }
    /// jump to subroutine
    /// `dst_addr` - Addressing Absoluteで指定されたJump先Address
    /// `opcode_addr` - JSR命令が格納されていたアドレス
    fn inst_jsr(&mut self, system: &mut System, dst_addr: u16, opcode_addr: u16) {
        let ret_addr = opcode_addr + 2;
        // pushはUpper, Lower
        self.stack_push(system, (ret_addr >>   8) as u8);
        self.stack_push(system, (ret_addr & 0xff) as u8);
        self.pc = dst_addr;
    }
    /// load accumulator
    fn inst_lda(&mut self, arg: u8) {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = arg;
    }
    /// load x register
    fn inst_ldx(&mut self, arg: u8) {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = arg;
    }
    /// load y register
    fn inst_ldy(&mut self, arg: u8) {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = arg;
    }
    /// logical shift right(Accumulator)
    fn inst_lsr_a(&mut self) {
        let result = self.a.wrapping_shr(1);

        let is_carry    = (self.a & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// logical shift right
    fn inst_lsr(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_shr(1);

        let is_carry    = (arg    & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr as usize, arg);
    }
    /// logical inclusive or
    fn inst_ora(&mut self, arg: u8) {
        let result = self.a | arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// push accumulator
    fn inst_pha(&mut self, system: &mut System) {
        self.stack_push(system, self.a);
    }
    /// push processor status
    fn inst_php(&mut self, system: &mut System) {
        self.stack_push(system, self.p);
    }
    /// pull accumulator
    fn inst_pla(&mut self, system: &mut System) {
        let result = self.stack_pop(system);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// pull processor status
    fn inst_plp(&mut self, system: &mut System) {
        self.p = self.stack_pop(system);
    }
    /// rorate left(Accumulator)
    fn inst_rol_a(&mut self) {
        let result = self.a.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

        let is_carry    = (self.a & 0x80) == 0x80;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// rorate left
    fn inst_rol(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

        let is_carry    = (arg    & 0x80) == 0x80;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr as usize, result);
    }
    /// rorate right(Accumulator)
    fn inst_ror_a(&mut self) {
        let result = self.a.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

        let is_carry    = (self.a & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// rorate right
    fn inst_ror(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

        let is_carry    = (arg & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr as usize, result);
    }
    /// return from interuppt
    fn inst_rti(&mut self, system: &mut System) {
        self.p = self.stack_pop(system);
        let pc_lower = self.stack_pop(system);
        let pc_upper = self.stack_pop(system);
        self.pc = ((pc_upper as u16) << 8) | (pc_lower as u16);
    }
    /// return from subroutine
    fn inst_rts(&mut self, system: &mut System) {
        let pc_lower = self.stack_pop(system);
        let pc_upper = self.stack_pop(system);
        self.pc = (((pc_upper as u16) << 8) | (pc_lower as u16)) + 1;
    }
    /// subtract with carry
    /// A = A-arg-(1-Carry)
    fn inst_sbc(&mut self, arg: u8) {
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
    }
    /// set carry flag
    fn inst_sec(&mut self) {
        self.write_carry_flag(true);
    }
    /// set decimal mode
    fn inst_sed(&mut self) {
        self.write_decimal_flag(true);
    }
    /// set interrupt disable
    fn inst_sei(&mut self) {
        self.write_interrupt_flag(true);
    }
    /// store accumulator
    fn inst_sta(&self, system: &mut System, dst_addr: u16) {
        system.write_u8(dst_addr as usize, self.a);
    }
    /// store x register
    fn inst_stx(&self, system: &mut System, dst_addr: u16) {
        system.write_u8(dst_addr as usize, self.x);
    }
    /// store y register
    fn inst_sty(&self, system: &mut System, dst_addr: u16) {
        system.write_u8(dst_addr as usize, self.y);
    }
    /// transfer accumulator to x
    fn inst_tax(&mut self) {
        let is_zero     = self.a == 0;
        let is_negative = (self.a & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = self.a;
    }
    /// transfer accumulator to y
    fn inst_tay(&mut self) {
        let is_zero     = self.a == 0;
        let is_negative = (self.a & 0x80) == 0x80;
        
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = self.a;
    }
    /// transfer stack pointer to x
    fn inst_tsx(&mut self) {
        let result = (self.sp & 0xff) as u8;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;
        
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
    }
    /// transfer x to accumulator
    fn inst_txa(&mut self) {
        let is_zero     = self.x == 0;
        let is_negative = (self.x & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = self.x;
    }
    /// transfer x to stack pointer
    fn inst_txs(&mut self) {
        // spの上位バイトは0x01固定
        // txsはstatus書き換えなし
        self.sp = (self.x as u16) | 0x0100u16; 
    }
    /// transfer y to accumulator
    fn inst_tya(&mut self) {
        let is_zero     = self.y == 0;
        let is_negative = (self.y & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = self.y;
    }
}
/// Fetch and Adressing Implementation
/// Accumulatorとimplicitは実装の必要なし
impl Cpu {
    /// #v
    fn addressing_immediate(&self, _system: &System, base_addr: u16)-> u16 {
        base_addr
    }
    /// a
    fn addressing_absolute(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr = base_addr;
        let upper_addr = base_addr + 1;
        let lower = system.read_u8(lower_addr as usize);
        let upper = system.read_u8(upper_addr as usize);
        let addr  = (lower as u16) | ((upper as u16) << 8);
        addr
    }
    /// (a) for JMP
    /// absolute indirect
    fn addressing_indirect(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr1 = base_addr;
        let upper_addr1 = base_addr + 1;
        let lower1 = system.read_u8(lower_addr1 as usize);
        let upper1 = system.read_u8(upper_addr1 as usize);
        let lower_addr2 = (lower1 as u16) | ((upper1 as u16) << 8);
        let upper_addr2 = lower_addr2.wrapping_add(1);
        let lower3 = system.read_u8(lower_addr2 as usize);
        let upper3 = system.read_u8(upper_addr2 as usize);
        let addr3 = (lower3 as u16) | ((upper3 as u16) << 8);
        addr3
    }
    /// d
    fn addressing_zero_page(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr = base_addr;
        let lower = system.read_u8(lower_addr as usize);
        let addr  = lower as u16;
        addr
    }
    /// d,x
    fn addressing_zero_page_indexed_x(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr = base_addr;
        let lower = system.read_u8(lower_addr as usize);
        let addr  = (lower as u16).wrapping_add(self.x as u16);
        addr
    }
    /// d,y
    fn addressing_zero_page_indexed_y(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr = base_addr;
        let lower = system.read_u8(lower_addr as usize);
        let addr  = (lower as u16).wrapping_add(self.y as u16);
        addr
    }
    /// a,x
    fn addressing_absolute_indexed_x(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr = base_addr;
        let upper_addr = base_addr + 1;
        let lower = system.read_u8(lower_addr as usize);
        let upper = system.read_u8(upper_addr as usize);
        let addr  = ((lower as u16) | ((upper as u16) << 8)).wrapping_add(self.x as u16);
        addr
    }
    /// a,y
    fn addressing_absolute_indexed_y(&self, system: &System, base_addr: u16)-> u16 {
        let lower_addr = base_addr;
        let upper_addr = base_addr + 1;
        let lower = system.read_u8(lower_addr as usize);
        let upper = system.read_u8(upper_addr as usize);
        let addr  = ((lower as u16) | ((upper as u16) << 8)).wrapping_add(self.y as u16);
        addr
    }
    /// label
    fn addressing_relative(&self, system: &System, base_addr: u16)-> u16 {
        let offset_addr = base_addr;
        let offset = system.read_u8(offset_addr as usize);
        let addr_signed  = ((offset as i8) as i32) + (self.pc as i32);
        assert!(addr_signed >= 0);
        assert!(addr_signed < 0x10000);
        let addr = addr_signed as u16;
        addr
    }
    /// (d,x)
    fn addressing_indexed_indirect(&self, system: &System, base_addr: u16)-> u16 {
        let addr1 = base_addr;
        let data1 = system.read_u8(addr1 as usize);
        let addr2 = (data1 as u16).wrapping_add(self.x as u16);
        let data2_lower = system.read_u8(addr2 as usize);
        let data2_upper = system.read_u8((addr2.wrapping_add(1)) as usize);
        let addr3 = (data2_lower as u16) | ((data2_upper as u16) << 8);
        addr3
    }
    /// (d),y
    fn addressing_indirect_indexed(&self, system: &System, base_addr: u16)-> u16 {
        let addr1_lower = base_addr;
        let addr1_upper = self.pc.wrapping_add(2);
        let data1_lower = system.read_u8(addr1_lower as usize);
        let data1_upper = system.read_u8(addr1_upper as usize);
        let addr2 = ((data1_lower as u16) | ((data1_upper as u16) << 8)) + (self.y as u16);
        addr2
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
    fn read_negative_flag(&self)  -> bool { (self.p & 0x80u8) != 0x00u8 }
    fn read_overflow_flag(&self)  -> bool { (self.p & 0x40u8) != 0x00u8 }
    fn read_reserved_flag(&self)  -> bool { (self.p & 0x20u8) != 0x00u8 }
    fn read_break_flag(&self)     -> bool { (self.p & 0x10u8) != 0x00u8 }
    fn read_decimal_flag(&self)   -> bool { (self.p & 0x08u8) != 0x00u8 }
    fn read_interrupt_flag(&self) -> bool { (self.p & 0x04u8) != 0x00u8 }
    fn read_zero_flag(&self)      -> bool { (self.p & 0x02u8) != 0x00u8 }
    fn read_carry_flag(&self)     -> bool { (self.p & 0x01u8) != 0x00u8 }

}