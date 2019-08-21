use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

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
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.a = arg;
        0
    }
    /// load x register
    pub fn inst_ldx(&mut self, arg: u8) -> u8 {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

        self.write_zero_flag(is_zero);
        self.write_negative_flag(is_negative);
        self.x = arg;
        0
    }
    /// load y register
    pub fn inst_ldy(&mut self, arg: u8) -> u8 {
        let is_zero     = arg == 0;
        let is_negative = (arg as i8) < 0;

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
