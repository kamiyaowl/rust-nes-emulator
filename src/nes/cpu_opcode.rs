use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus, EmulateControl};

/// Decode and Run
impl Cpu {
    /// 1命令実行します
    /// return: かかったclock cycle count`
    pub fn step(&mut self, system: &mut System) -> u8 {
        let opcode = system.read_u8(self.pc);
        self.increment_pc(1);

        let cycles = match opcode {
            0x69 => {
                // adc immediate
                let addr = self.addressing_immediate(self.pc);
                let data = system.read_u8(addr);
                self.increment_pc(1);
                self.inst_adc(data);
                2
            }
            _ => {
                panic!("invalid Operation. opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", opcode, self.pc, self.a, self.x, self.y, self.sp, self.p);
            }
        };
        cycles
    }

}

