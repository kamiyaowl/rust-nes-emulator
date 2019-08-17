use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

/// Instruction Implementation
/// http://obelisk.me.uk/6502/reference.html
impl Cpu {
    /// add with carry
    pub fn inst_adc(&mut self, arg: u8) {
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
    pub fn inst_and(&mut self, arg: u8) {
        let result = self.a & arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// arithmetic shift left(Accumulator)
    pub fn inst_asl_a(&mut self) {
        let (result, is_carry) = self.a.overflowing_shl(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// arithmetic shift left
    pub fn inst_asl(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let (result, is_carry) = arg.overflowing_shl(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result);
    }
    /// branch if carry clear
    pub fn inst_bcc(&mut self, arg: u8) {
        if !self.read_carry_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if carry set
    pub fn inst_bcs(&mut self, arg: u8) {
        if self.read_carry_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if equal
    pub fn inst_beq(&mut self, arg: u8) {
        if self.read_zero_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// bit test
    pub fn inst_bit(&mut self, arg: u8) {
        let is_negative = (arg & 0x80) == 0x80;
        let is_overflow = (arg & 0x40) == 0x40;
        let is_zero     = is_negative && is_overflow;

        self.write_negative_flag(is_negative);
        self.write_zero_flag(is_zero);
        self.write_overflow_flag(is_overflow);
    }
    /// branch if minus
    pub fn inst_bmi(&mut self, arg: u8) {
        if self.read_negative_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if not equal
    pub fn inst_bne(&mut self, arg: u8) {
        if !self.read_zero_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if plus
    pub fn inst_bpl(&mut self, arg: u8) {
        if !self.read_negative_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// force interrupt
    pub fn inst_brk(&mut self, system: &mut System) {
        self.write_break_flag(true);
        self.interrupt(system, Interrupt::BRK);
    }
    /// branch if overflow clear
    pub fn inst_bvc(&mut self, arg: u8) {
        if !self.read_overflow_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// branch if overflow set
    pub fn inst_bvs(&mut self, arg: u8) {
        if self.read_overflow_flag() {
            self.pc = 0x0100u16 | (arg as u16);
        }
    }
    /// clear carry flag
    pub fn inst_clc(&mut self) {
        self.write_carry_flag(false);
    }
    /// clear decimal mode
    pub fn inst_cld(&mut self) {
        self.write_decimal_flag(false);
    }
    /// clear interrupt disable
    pub fn inst_cli(&mut self) {
        self.write_interrupt_flag(false);
    }
    /// clear overflow flag
    pub fn inst_clv(&mut self) {
        self.write_overflow_flag(false);
    }
    /// compare
    pub fn inst_cmp(&mut self, arg: u8) {
        let (result, is_carry) = self.a.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
    }
    /// compare x register
    pub fn inst_cpx(&mut self, arg: u8) {
        let (result, is_carry) = self.x.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
    }
    /// compare y register
    pub fn inst_cpy(&mut self, arg: u8) {
        let (result, is_carry) = self.y.overflowing_sub(arg);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
    }
    /// decrement memory
    pub fn inst_dec(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result);
    }
    /// decrement x register
    pub fn inst_dex(&mut self) {
        let result = self.x.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
    }
    /// decrement y register
    pub fn inst_dey(&mut self) {
        let result = self.y.wrapping_sub(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = result;
    }
    /// exclusive or
    pub fn inst_eor(&mut self, arg: u8) {
        let result =self.a ^ arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// increment memory
    pub fn inst_inc(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result);
    }
    /// increment x register
    pub fn inst_inx(&mut self) {
        let result = self.x.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
    }
    /// increment y register
    pub fn inst_iny(&mut self) {
        let result = self.y.wrapping_add(1);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = result;
    }
    /// jump
    /// `dst_addr` - Addressing Absolute/Indirectで指定されたJump先Address
    pub fn inst_jmp(&mut self, dst_addr: u16) {
        self.pc = dst_addr;
    }
    /// jump to subroutine
    /// `dst_addr` - Addressing Absoluteで指定されたJump先Address
    /// `opcode_addr` - JSR命令が格納されていたアドレス
    pub fn inst_jsr(&mut self, system: &mut System, dst_addr: u16, opcode_addr: u16) {
        let ret_addr = opcode_addr + 2;
        // pushはUpper, Lower
        self.stack_push(system, (ret_addr >>   8) as u8);
        self.stack_push(system, (ret_addr & 0xff) as u8);
        self.pc = dst_addr;
    }
    /// load accumulator
    pub fn inst_lda(&mut self, arg: u8) {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = arg;
    }
    /// load x register
    pub fn inst_ldx(&mut self, arg: u8) {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = arg;
    }
    /// load y register
    pub fn inst_ldy(&mut self, arg: u8) {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = arg;
    }
    /// logical shift right(Accumulator)
    pub fn inst_lsr_a(&mut self) {
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
    pub fn inst_lsr(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_shr(1);

        let is_carry    = (arg    & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, arg);
    }
    /// logical inclusive or
    pub fn inst_ora(&mut self, arg: u8) {
        let result = self.a | arg;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// push accumulator
    pub fn inst_pha(&mut self, system: &mut System) {
        self.stack_push(system, self.a);
    }
    /// push processor status
    pub fn inst_php(&mut self, system: &mut System) {
        self.stack_push(system, self.p);
    }
    /// pull accumulator
    pub fn inst_pla(&mut self, system: &mut System) {
        let result = self.stack_pop(system);

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = result;
    }
    /// pull processor status
    pub fn inst_plp(&mut self, system: &mut System) {
        self.p = self.stack_pop(system);
    }
    /// rorate left(Accumulator)
    pub fn inst_rol_a(&mut self) {
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
    pub fn inst_rol(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_shl(1) | (if self.read_carry_flag() { 0x01 } else { 0x00 } );

        let is_carry    = (arg    & 0x80) == 0x80;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result);
    }
    /// rorate right(Accumulator)
    pub fn inst_ror_a(&mut self) {
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
    pub fn inst_ror(&mut self, system: &mut System, dst_addr: u16, arg: u8) {
        let result = arg.wrapping_shr(1) | (if self.read_carry_flag() { 0x80 } else { 0x00 } );

        let is_carry    = (arg & 0x01) == 0x01;
        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;

        self.write_carry_flag(is_carry);
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        system.write_u8(dst_addr, result);
    }
    /// return from interuppt
    pub fn inst_rti(&mut self, system: &mut System) {
        self.p = self.stack_pop(system);
        let pc_lower = self.stack_pop(system);
        let pc_upper = self.stack_pop(system);
        self.pc = ((pc_upper as u16) << 8) | (pc_lower as u16);
    }
    /// return from subroutine
    pub fn inst_rts(&mut self, system: &mut System) {
        let pc_lower = self.stack_pop(system);
        let pc_upper = self.stack_pop(system);
        self.pc = (((pc_upper as u16) << 8) | (pc_lower as u16)) + 1;
    }
    /// subtract with carry
    /// A = A-arg-(1-Carry)
    pub fn inst_sbc(&mut self, arg: u8) {
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
    pub fn inst_sec(&mut self) {
        self.write_carry_flag(true);
    }
    /// set decimal mode
    pub fn inst_sed(&mut self) {
        self.write_decimal_flag(true);
    }
    /// set interrupt disable
    pub fn inst_sei(&mut self) {
        self.write_interrupt_flag(true);
    }
    /// store accumulator
    pub fn inst_sta(&self, system: &mut System, dst_addr: u16) {
        system.write_u8(dst_addr, self.a);
    }
    /// store x register
    pub fn inst_stx(&self, system: &mut System, dst_addr: u16) {
        system.write_u8(dst_addr, self.x);
    }
    /// store y register
    pub fn inst_sty(&self, system: &mut System, dst_addr: u16) {
        system.write_u8(dst_addr, self.y);
    }
    /// transfer accumulator to x
    pub fn inst_tax(&mut self) {
        let is_zero     = self.a == 0;
        let is_negative = (self.a & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = self.a;
    }
    /// transfer accumulator to y
    pub fn inst_tay(&mut self) {
        let is_zero     = self.a == 0;
        let is_negative = (self.a & 0x80) == 0x80;
        
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.y = self.a;
    }
    /// transfer stack pointer to x
    pub fn inst_tsx(&mut self) {
        let result = (self.sp & 0xff) as u8;

        let is_zero     = result == 0;
        let is_negative = (result & 0x80) == 0x80;
        
        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = result;
    }
    /// transfer x to accumulator
    pub fn inst_txa(&mut self) {
        let is_zero     = self.x == 0;
        let is_negative = (self.x & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = self.x;
    }
    /// transfer x to stack pointer
    pub fn inst_txs(&mut self) {
        // spの上位バイトは0x01固定
        // txsはstatus書き換えなし
        self.sp = (self.x as u16) | 0x0100u16; 
    }
    /// transfer y to accumulator
    pub fn inst_tya(&mut self) {
        let is_zero     = self.y == 0;
        let is_negative = (self.y & 0x80) == 0x80;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = self.y;
    }
}
