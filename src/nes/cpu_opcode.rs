use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus, EmulateControl};

/// inst! macro
/// 命令のアドレッシング、フェッチ、pcのincrement、実行、クロックサイクルの返却をまとめて行います
macro_rules! inst {
    (
        $self:expr, $system:expr,
        $name:expr, cycle => $cycle:expr, pc_incr => $pc_incr:expr,
        $addressing_closure:expr,
        $inst_closure:expr
    ) => {
        {
            if cfg!(debug_assertions) {
                println!("[before][#{}] cycle:{} pc_incr:{} pc{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            let addr = $addressing_closure();
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
        $self:expr, $system:expr, $target_opcode:expr,
        $(
            {
                $name:expr, opcode => $opcode:expr, cycle => $cycle:expr, pc_incr => $pc_incr:expr,
                $addressing_closure:expr,
                $inst_closure:expr
            }
        ),*
    ) => {
        match $target_opcode {
            $(
                $opcode => inst!($self, $system,
                    $name, cycle => $cycle, pc_incr => $pc_incr,
                    $addressing_closure,
                    $inst_closure
                ),
            )*
            _ => {
                panic!("invalid Operation. opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $target_opcode, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            },
        }
    }

}



/// Decode and Run
impl Cpu {
    /// 1命令実行します
    /// return: かかったclock cycle count`
    pub fn step(&mut self, system: &mut System) -> u8 {
        let opcode = system.read_u8(self.pc);
        self.increment_pc(1);
        if cfg!(debug_assertions) {
            println!("[opcode fetched] opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", opcode, self.pc, self.a, self.x, self.y, self.sp, self.p);
        }
        let cycles = inst!(self, system, opcode,
            {
                "Dummy", opcode => 0x00, cycle => 1, pc_incr => 1, 
                || self.addressing_immediate(system, self.pc),
                |addr, data| println!("Hello macro! addr:{:04x} data:{:02x}", addr, data)
            },
            {
                "ADC imm", opcode => 0x69, cycle => 2, pc_incr => 1, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_adc(data)
            }
        );
        cycles
    }

}

