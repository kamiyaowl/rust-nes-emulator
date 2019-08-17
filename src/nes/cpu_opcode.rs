use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus, EmulateControl};

/// inst! macro
/// 命令のアドレッシング、フェッチ、pcのincrement、実行、クロックサイクルの返却をまとめて行います
macro_rules! inst {
    (
        $self:expr, $system:expr,
        $name:expr, cycle => $cycle:expr, pc_incr => $pc_incr:expr,
        $adressing_closure:expr,
        $inst_closure:expr
    ) => {
        {
            if cfg!(debug_assertions) {
                println!("[before][#{}] cycle:{} pc_incr:{} pc{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            let addr = $adressing_closure();
            let data = $system.read_u8(addr);
            $self.increment_pc($pc_incr);
            $inst_closure(addr, data);
            if cfg!(debug_assertions) {
                println!("[after][#{}] cycle:{} pc_incr:{} pc{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            $cycle
        }
    };
    (
        $self:expr, $system:expr,
        [
            $(
                $opcode => {
                    $name:expr, cycle => $cycle:expr, pc_incr => $pc_incr:expr,
                    $adressing_closure:expr,
                    $inst_closure:expr
                }
            ),*
        ]
    ) => ()

}



/// Decode and Run
impl Cpu {
    /// 1命令実行します
    /// return: かかったclock cycle count`
    pub fn step(&mut self, system: &mut System) -> u8 {
        let opcode = system.read_u8(self.pc);
        if cfg!(debug_assertions) {
            println!("[opcode fetched] opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", opcode, self.pc, self.a, self.x, self.y, self.sp, self.p);
        }

        self.increment_pc(1);

        let cycles = match opcode {
            /**************** ADC ****************/
            0x69 => inst!(self, system,
                "ADC imm", cycle => 2, pc_incr => 1, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_adc(data)
            ),
            // 0x69 => {
            //     let addr = self.addressing_immediate(system, self.pc);
            //     let data = system.read_u8(addr);
            //     self.increment_pc(1);
            //     self.inst_adc(data);
            //     2
            // },
            0x65 => {
                let addr = self.addressing_zero_page(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(1);
                self.inst_adc(data);
                3
            },
            0x75 => {
                let addr = self.addressing_zero_page_indexed_x(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(1);
                self.inst_adc(data);
                4
            },
            0x6d => {
                let addr = self.addressing_absolute(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(2);
                self.inst_adc(data);
                4
            },
            0x7d => {
                let addr = self.addressing_absolute_indexed_x(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(2);
                self.inst_adc(data);
                4
            },
            0x79 => {
                let addr = self.addressing_absolute_indexed_y(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(2);
                self.inst_adc(data);
                4
            },
            0x61 => {
                let addr = self.addressing_indexed_indirect(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(1);
                self.inst_adc(data);
                6
            },
            0x71 => {
                let addr = self.addressing_indirect_indexed(system, self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(1);
                self.inst_adc(data);
                5
            },
            /**************** AND ****************/
            /**************** ASL ****************/
            /**************** BCC ****************/
            /**************** BCS ****************/
            /**************** BEQ ****************/
            /**************** BIT ****************/
            /**************** BMI ****************/
            /**************** BNE ****************/
            /**************** BPL ****************/
            /**************** BRK ****************/
            /**************** BVC ****************/
            /**************** BVS ****************/
            /**************** CLC ****************/
            /**************** CLD ****************/
            /**************** CLI ****************/
            /**************** CLV ****************/
            /**************** CMP ****************/
            /**************** CPX ****************/
            /**************** CPY ****************/
            /**************** DEC ****************/
            /**************** DEX ****************/
            /**************** DEY ****************/
            /**************** EOR ****************/
            /**************** INC ****************/
            /**************** INX ****************/
            /**************** INY ****************/
            /**************** JMP ****************/
            /**************** JSR ****************/
            /**************** LDA ****************/
            /**************** LDX ****************/
            /**************** LDY ****************/
            /**************** LSR ****************/
            /**************** NOP ****************/
            /**************** ORA ****************/
            /**************** PHA ****************/
            /**************** PHP ****************/
            /**************** PLA ****************/
            /**************** PLP ****************/
            /**************** ROL ****************/
            /**************** ROR ****************/
            /**************** RTI ****************/
            /**************** RTS ****************/
            /**************** SBC ****************/
            /**************** SEC ****************/
            /**************** SED ****************/
            /**************** SEI ****************/
            /**************** STA ****************/
            /**************** STX ****************/
            /**************** STY ****************/
            /**************** TAX ****************/
            /**************** TAY ****************/
            /**************** TSX ****************/
            /**************** TXA ****************/
            /**************** TXS ****************/
            /**************** TYA ****************/

            /*********** unimplemented ***********/
            _ => {
                panic!("invalid Operation. opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", opcode, self.pc, self.a, self.x, self.y, self.sp, self.p);
            },
        };
        cycles
    }

}

