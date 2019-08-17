use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

/// inst! macro
/// 命令のアドレッシング、フェッチ、pcのincrement、実行、クロックサイクルの返却をまとめて行います
macro_rules! inst {
    (
        $self:expr, $system:expr,
        $name:expr, pc_incr => $pc_incr:expr, cycle => $cycle:expr, 
        $addressing_closure:expr,
        $inst_closure:expr
    ) => {
        {
            if cfg!(debug_assertions) {
                println!("[#{}][before] cycle:{} pc_incr:{} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            // fetchしない場合(accumulate, implicit)は、pc incrementを0に設定する
            // addressはそのまま供給する
            if $pc_incr > 0 {
                let addr = $addressing_closure();
                let data = $system.read_u8(addr);
                $self.increment_pc($pc_incr);

                if cfg!(debug_assertions) {
                    println!("[#{}][addressing] addr:{:04x} data:{:02x}", $name, addr, data);
                }
                $inst_closure(addr, data);
            } else {
                if cfg!(debug_assertions) {
                    println!("[#{}][addressing] skip addressing", $name);
                }
                // for implicit, accumulate
                $inst_closure(0, 0);
            }
            if cfg!(debug_assertions) {
                println!("[#{}][after ] cycle:{} pc_incr:{} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            $cycle
        }
    };
    (
        $self:expr, $system:expr, $target_opcode:expr,
        $(
            {
                $name:expr, opcode => $opcode:expr,pc_incr => $pc_incr:expr, cycle => $cycle:expr, 
                $addressing_closure:expr,
                $inst_closure:expr
            }
        ),*
    ) => {
        match $target_opcode {
            $(
                $opcode => inst!($self, $system,
                    $name, pc_incr => $pc_incr, cycle => $cycle,
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
        inst!(self, system, opcode,
            /**************** ADC ****************/
            {
                "ADC imm",
                opcode => 0x69, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC zero page", 
                opcode => 0x65, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC zero page x", 
                opcode => 0x75, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute", 
                opcode => 0x6d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute x", 
                opcode => 0x7d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute y", 
                opcode => 0x79, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC indirect x", 
                opcode => 0x61, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC indirect y", 
                opcode => 0x71, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            /**************** AND ****************/
            {
                "AND imm",
                opcode => 0x29, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND zero page", 
                opcode => 0x25, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND zero page x", 
                opcode => 0x35, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute", 
                opcode => 0x2d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute x", 
                opcode => 0x3d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute y", 
                opcode => 0x39, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND indirect x", 
                opcode => 0x21, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND indirect y", 
                opcode => 0x31, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            /**************** ASL ****************/
            {
                "ASL accumulator", 
                opcode => 0x0a, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_asl_a()
            },
            {
                "ASL zero page", 
                opcode => 0x06, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            {
                "ASL zero page x", 
                opcode => 0x16, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            {
                "ASL absolute", 
                opcode => 0x0e, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            {
                "ASL absolute x", 
                opcode => 0x1e, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            /**************** BCC ****************/
            {
                "BCC relative", 
                opcode => 0x90, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bcc(data)
            },
            /**************** BCS ****************/
            {
                "BCS relative", 
                opcode => 0xb0, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bcs(data)
            },
            /**************** BEQ ****************/
            {
                "BEQ relative", 
                opcode => 0xf0, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_beq(data)
            },
            /**************** BIT ****************/
            {
                "BIT zero page", 
                opcode => 0x24, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_bit(data)
            },
            {
                "BIT absolute", 
                opcode => 0x2c, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_bit(data)
            },
            /**************** BMI ****************/
            {
                "BMI relative", 
                opcode => 0x30, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bmi(data)
            },
            /**************** BNE ****************/
            {
                "BNE relative", 
                opcode => 0xd0, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bne(data)
            },
            /**************** BPL ****************/
            {
                "BPL relative", 
                opcode => 0x10, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bpl(data)
            },
            /**************** BRK ****************/
            {
                "BRK implied", 
                opcode => 0x00, pc_incr => 0, cycle => 7, 
                || 0,
                |_addr, _data| self.inst_brk(system)
            },
            /**************** BVC ****************/
            {
                "BVC relative", 
                opcode => 0x50, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bvc(data)
            },
            /**************** BVS ****************/
            {
                "BVS relative", 
                opcode => 0x70, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |_addr, data| self.inst_bvs(data)
            },
            /**************** CLC ****************/
            {
                "CLC implied", 
                opcode => 0x18, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_clc()
            },
            /**************** CLD ****************/
            {
                "CLD implied", 
                opcode => 0xd8, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_cld()
            },
            /**************** CLI ****************/
            {
                "CLI implied", 
                opcode => 0x58, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_cli()
            },
            /**************** CLV ****************/
            {
                "CLV implied", 
                opcode => 0xb8, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_clv()
            },
            /**************** CMP ****************/
            {
                "CMP imm",
                opcode => 0xc9, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP zero page", 
                opcode => 0xc5, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP zero page x", 
                opcode => 0xd5, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP absolute", 
                opcode => 0xcd, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP absolute x", 
                opcode => 0xdd, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP absolute y", 
                opcode => 0xd9, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP indirect x", 
                opcode => 0xc1, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP indirect y", 
                opcode => 0xd1, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            /**************** CPX ****************/
            {
                "CMX imm",
                opcode => 0xe0, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_cpx(data)
            },
            {
                "CMX zero page",
                opcode => 0xe4, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_cpx(data)
            },
            {
                "CMX absolute",
                opcode => 0xec, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_cpx(data)
            },
            /**************** CPY ****************/
            {
                "CMX imm",
                opcode => 0xc0, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_cpy(data)
            },
            {
                "CMX zero page",
                opcode => 0xc4, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_cpy(data)
            },
            {
                "CMX absolute",
                opcode => 0xcc, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_cpy(data)
            },
            /**************** DEC ****************/
            {
                "DEC zero page",
                opcode => 0xc6, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            {
                "DEC zero page x",
                opcode => 0xd6, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            {
                "DEC absolute",
                opcode => 0xce, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            {
                "DEC absolute x",
                opcode => 0xde, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            /**************** DEX ****************/
            {
                "DEX implied",
                opcode => 0xca, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_dex()
            },
            /**************** DEY ****************/
            {
                "DEY implied",
                opcode => 0x88, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_dey()
            },
            /**************** EOR ****************/
            {
                "EOR imm",
                opcode => 0x49, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR zero page", 
                opcode => 0x45, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR zero page x", 
                opcode => 0x55, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR absolute", 
                opcode => 0x4d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR absolute x", 
                opcode => 0x5d, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR absolute y", 
                opcode => 0x59, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR indirect x", 
                opcode => 0x41, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR indirect y", 
                opcode => 0x51, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            /**************** INC ****************/
            {
                "INC zero page",
                opcode => 0xe6, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            {
                "INC zero page x",
                opcode => 0xf6, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            {
                "INC absolute",
                opcode => 0xee, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            {
                "INC absolute x",
                opcode => 0xfe, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            /**************** INX ****************/
            {
                "INX implied",
                opcode => 0xe8, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_inx()
            },
            /**************** INY ****************/
            {
                "INY implied",
                opcode => 0xc8, pc_incr => 0, cycle => 2, 
                || 0,
                |_addr, _data| self.inst_iny()
            },
            /**************** JMP ****************/
            {
                "JMP absolute",
                opcode => 0x4c, pc_incr => 2, cycle => 3, 
                || self.addressing_absolute(system, self.pc),
                |addr, _data| self.inst_jmp(addr)
            },
            {
                "JMP indirect",
                opcode => 0x6c, pc_incr => 2, cycle => 5, 
                || self.addressing_indirect_x(system, self.pc),
                |addr, _data| self.inst_jmp(addr)
            },
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


            {
                "Dummy", opcode => 0xff, pc_incr => 1, cycle => 1,
                || self.addressing_immediate(system, self.pc),
                |addr, data| println!("Hello macro! addr:{:04x} data:{:02x}", addr, data)
            }
        )
    }

}

