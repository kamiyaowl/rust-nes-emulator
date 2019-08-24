use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

/// inst! macro
/// 命令のアドレッシング、フェッチ、pcのincrement、実行、クロックサイクルの返却をまとめて行います
macro_rules! inst {
    (
        $self:expr, $system:expr,
        $name:expr, non_destructive => $is_nondestructive:expr, pc_incr => $pc_incr:expr, cycle => $cycle:expr, 
        $addressing_func:expr,
        $inst_func:expr
    ) => {
        {
            if cfg!(debug_cpu) && cfg!(debug_assertions) && cfg!(not(no_std)) {
                println!("[#{}][before] cycle:{} pc_incr:{} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}(NO*BDIZC)", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            // fetchしない場合(accumulate, implicit)は、pc incrementを0に設定する
            // addressはそのまま供給する
            let (addr, additional_cycle1, data) = if $pc_incr > 0 {
                    let (a, c) = $addressing_func();
                    let d      = $system.read_u8(a, $is_nondestructive);
                    // addressingした分進めとく
                    $self.increment_pc($pc_incr);
                    // debug print
                    if  cfg!(debug_cpu) && cfg!(debug_assertions) && cfg!(not(no_std)) {
                        println!("[#{}][addressing] addr:{:04x} data:{:02x} | data(char):{}", $name, a, d, d as char);
                    }
                    (a, c, d)
                } else {
                    // debug print
                    if  cfg!(debug_cpu)  && cfg!(debug_assertions) && cfg!(not(no_std)) {
                        println!("[#{}][addressing] skip addressing", $name);
                    }
                    // implied, accumulator
                    (0u16, 0u8, 0u8)
                };
            // 命令実行
            let additional_cycle2 = $inst_func(addr, data);
            if cfg!(debug_cpu) && cfg!(debug_assertions) && cfg!(not(no_std)) {
                println!("[#{}][after ] cycle:{} pc_incr:{} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}(NO*BDIZC)", $name, $cycle, $pc_incr, $self.pc, $self.a, $self.x, $self.y, $self.sp, $self.p);
            }
            // かかるclock cycleを返却
            ($cycle + additional_cycle1 + additional_cycle2)
        }
    };
    (
        $self:expr, $system:expr, $target_opcode:expr,
        $(
            {
                $name:expr, opcode => $opcode:expr, non_destructive => $is_nondestructive:expr, pc_incr => $pc_incr:expr, cycle => $cycle:expr, 
                $addressing_func:expr,
                $inst_func:expr
            }
        ),*
    ) => {
        match $target_opcode {
            $(
                $opcode => inst!($self, $system,
                    $name, non_destructive => $is_nondestructive, pc_incr => $pc_incr, cycle => $cycle,
                    $addressing_func,
                    $inst_func
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
        let opcode = system.read_u8(self.pc, false);
        self.increment_pc(1);
        if cfg!(debug_cpu) &&  cfg!(debug_assertions) && cfg!(not(no_std)) {
            println!("[opcode fetched] opcode:{:02x} pc:{:04x} a:{:02x} x:{:02x} y:{:02x} sp:{:04x} p:{:08b}", opcode, self.pc, self.a, self.x, self.y, self.sp, self.p);
        }
        inst!(self, system, opcode,
            /**************** ADC ****************/
            {
                "ADC imm",
                opcode => 0x69, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC zero page", 
                opcode => 0x65, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC zero page x", 
                opcode => 0x75, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute", 
                opcode => 0x6d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute x", 
                opcode => 0x7d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC absolute y", 
                opcode => 0x79, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC indirect x", 
                opcode => 0x61, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            {
                "ADC indirect y", 
                opcode => 0x71, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_adc(data)
            },
            /**************** AND ****************/
            {
                "AND imm",
                opcode => 0x29, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND zero page", 
                opcode => 0x25, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND zero page x", 
                opcode => 0x35, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute", 
                opcode => 0x2d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute x", 
                opcode => 0x3d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND absolute y", 
                opcode => 0x39, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND indirect x", 
                opcode => 0x21, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            {
                "AND indirect y", 
                opcode => 0x31, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_and(data)
            },
            /**************** ASL ****************/
            {
                "ASL accumulator", 
                opcode => 0x0a, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_asl_a()
            },
            {
                "ASL zero page", 
                opcode => 0x06, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            {
                "ASL zero page x", 
                opcode => 0x16, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            {
                "ASL absolute", 
                opcode => 0x0e, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            {
                "ASL absolute x", 
                opcode => 0x1e, non_destructive => false, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_asl(system, addr, data)
            },
            /**************** BCC ****************/
            {
                "BCC relative", 
                opcode => 0x90, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bcc(addr)
            },
            /**************** BCS ****************/
            {
                "BCS relative", 
                opcode => 0xb0, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bcs(addr)
            },
            /**************** BEQ ****************/
            {
                "BEQ relative", 
                opcode => 0xf0, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_beq(addr)
            },
            /**************** BIT ****************/
            // non destructive readが必要
            {
                "BIT zero page", 
                opcode => 0x24, non_destructive => true  , pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_bit(data)
            },
            {
                "BIT absolute", 
                opcode => 0x2c, non_destructive => true  , pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_bit(data)
            },
            /**************** BMI ****************/
            {
                "BMI relative", 
                opcode => 0x30, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bmi(addr)
            },
            /**************** BNE ****************/
            {
                "BNE relative", 
                opcode => 0xd0, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bne(addr)
            },
            /**************** BPL ****************/
            {
                "BPL relative", 
                opcode => 0x10, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bpl(addr)
            },
            /**************** BRK ****************/
            {
                "BRK implied", 
                opcode => 0x00, non_destructive => false, pc_incr => 0, cycle => 7, 
                || (0, 0),
                |_addr, _data| self.inst_brk(system)
            },
            /**************** BVC ****************/
            {
                "BVC relative", 
                opcode => 0x50, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bvc(addr)
            },
            /**************** BVS ****************/
            {
                "BVS relative", 
                opcode => 0x70, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_relative(system, self.pc),
                |addr, _data| self.inst_bvs(addr)
            },
            /**************** CLC ****************/
            {
                "CLC implied", 
                opcode => 0x18, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_clc()
            },
            /**************** CLD ****************/
            {
                "CLD implied", 
                opcode => 0xd8, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_cld()
            },
            /**************** CLI ****************/
            {
                "CLI implied", 
                opcode => 0x58, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_cli()
            },
            /**************** CLV ****************/
            {
                "CLV implied", 
                opcode => 0xb8, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_clv()
            },
            /**************** CMP ****************/
            {
                "CMP imm",
                opcode => 0xc9, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP zero page", 
                opcode => 0xc5, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP zero page x", 
                opcode => 0xd5, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP absolute", 
                opcode => 0xcd, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP absolute x", 
                opcode => 0xdd, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP absolute y", 
                opcode => 0xd9, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP indirect x", 
                opcode => 0xc1, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            {
                "CMP indirect y", 
                opcode => 0xd1, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_cmp(data)
            },
            /**************** CPX ****************/
            {
                "CMX imm",
                opcode => 0xe0, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_cpx(data)
            },
            {
                "CMX zero page",
                opcode => 0xe4, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_cpx(data)
            },
            {
                "CMX absolute",
                opcode => 0xec, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_cpx(data)
            },
            /**************** CPY ****************/
            {
                "CMX imm",
                opcode => 0xc0, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_cpy(data)
            },
            {
                "CMX zero page",
                opcode => 0xc4, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_cpy(data)
            },
            {
                "CMX absolute",
                opcode => 0xcc, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_cpy(data)
            },
            /**************** DEC ****************/
            {
                "DEC zero page",
                opcode => 0xc6, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            {
                "DEC zero page x",
                opcode => 0xd6, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            {
                "DEC absolute",
                opcode => 0xce, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            {
                "DEC absolute x",
                opcode => 0xde, non_destructive => false, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_dec(system, addr, data)
            },
            /**************** DEX ****************/
            {
                "DEX implied",
                opcode => 0xca, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_dex()
            },
            /**************** DEY ****************/
            {
                "DEY implied",
                opcode => 0x88, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_dey()
            },
            /**************** EOR ****************/
            {
                "EOR imm",
                opcode => 0x49, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR zero page", 
                opcode => 0x45, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR zero page x", 
                opcode => 0x55, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR absolute", 
                opcode => 0x4d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR absolute x", 
                opcode => 0x5d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR absolute y", 
                opcode => 0x59, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR indirect x", 
                opcode => 0x41, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            {
                "EOR indirect y", 
                opcode => 0x51, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_eor(data)
            },
            /**************** INC ****************/
            {
                "INC zero page",
                opcode => 0xe6, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            {
                "INC zero page x",
                opcode => 0xf6, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            {
                "INC absolute",
                opcode => 0xee, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            {
                "INC absolute x",
                opcode => 0xfe, non_destructive => false, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_inc(system, addr, data)
            },
            /**************** INX ****************/
            {
                "INX implied",
                opcode => 0xe8, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_inx()
            },
            /**************** INY ****************/
            {
                "INY implied",
                opcode => 0xc8, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_iny()
            },
            /**************** JMP ****************/
            {
                "JMP absolute",
                opcode => 0x4c, non_destructive => false, pc_incr => 2, cycle => 3, 
                || self.addressing_absolute(system, self.pc),
                |addr, _data| self.inst_jmp(addr)
            },
            {
                "JMP indirect",
                opcode => 0x6c, non_destructive => false, pc_incr => 2, cycle => 5, 
                || self.addressing_indirect(system, self.pc),
                |addr, _data| self.inst_jmp(addr)
            },
            /**************** JSR ****************/
            {
                "JSR absolute",
                opcode => 0x20, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, _data| {
                    // JSR命令が入っていたアドレスは、pcをすでに進めてしまっているので再計算
                    let opcode_addr = self.pc - 3;
                    self.inst_jsr(system, addr, opcode_addr)
                }
            },
            /**************** LDA ****************/
            {
                "LDA imm",
                opcode => 0xa9, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA zero page", 
                opcode => 0xa5, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA zero page x", 
                opcode => 0xb5, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA absolute", 
                opcode => 0xad, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA absolute x", 
                opcode => 0xbd, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA absolute y", 
                opcode => 0xb9, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA indirect x", 
                opcode => 0xa1, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            {
                "LDA indirect y", 
                opcode => 0xb1, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_lda(data)
            },
            /**************** LDX ****************/
            {
                "LDX imm",
                opcode => 0xa2, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_ldx(data)
            },
            {
                "LDX zero page",
                opcode => 0xa6, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_ldx(data)
            },
            {
                "LDX zero page y",
                opcode => 0xb6, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_y(system, self.pc),
                |_addr, data| self.inst_ldx(data)
            },
            {
                "LDX absolute",
                opcode => 0xae, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_ldx(data)
            },
            {
                "LDX absolute y",
                opcode => 0xbe, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_ldx(data)
            },
            /**************** LDY ****************/
            {
                "LDY imm",
                opcode => 0xa0, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_ldy(data)
            },
            {
                "LDY zero page",
                opcode => 0xa4, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_ldy(data)
            },
            {
                "LDY zero page x",
                opcode => 0xb4, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_ldy(data)
            },
            {
                "LDY absolute",
                opcode => 0xac, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_ldy(data)
            },
            {
                "LDY absolute x",
                opcode => 0xbc, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_ldy(data)
            },
            /**************** LSR ****************/
            {
                "LSR accumulator",
                opcode => 0x4a, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_lsr_a()
            },
            {
                "LSR zero page",
                opcode => 0x46, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_lsr(system, addr, data)
            },
            {
                "LSR zero page x",
                opcode => 0x56, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_lsr(system, addr, data)
            },
            {
                "LSR absolute",
                opcode => 0x4e, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_lsr(system, addr, data)
            },
            {
                "LSR absolute x",
                opcode => 0x5e, non_destructive => false, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_lsr(system, addr, data)
            },
            /**************** NOP ****************/
            {
                "NOP implied",
                opcode => 0xea, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_nop()
            },
            /**************** ORA ****************/
            {
                "ORA imm",
                opcode => 0x09, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA zero page", 
                opcode => 0x05, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA zero page x", 
                opcode => 0x15, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA absolute", 
                opcode => 0x0d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA absolute x", 
                opcode => 0x1d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA absolute y", 
                opcode => 0x19, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA indirect x", 
                opcode => 0x01, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            {
                "ORA indirect y", 
                opcode => 0x11, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_ora(data)
            },
            /**************** PHA ****************/
            {
                "PHA implied",
                opcode => 0x48, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_pha(system)
            },
            /**************** PHP ****************/
            {
                "PHP implied",
                opcode => 0x08, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_php(system)
            },
            /**************** PLA ****************/
            {
                "PLA implied",
                opcode => 0x68, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_pla(system)
            },
            /**************** PLP ****************/
            {
                "PLP implied",
                opcode => 0x28, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_plp(system)
            },
            /**************** ROL ****************/
            {
                "ROL accumulator",
                opcode => 0x2a, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_rol_a()
            },
            {
                "ROL zero page",
                opcode => 0x26, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_rol(system, addr, data)
            },
            {
                "ROL zero page x",
                opcode => 0x36, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_rol(system, addr, data)
            },
            {
                "ROL absolute",
                opcode => 0x2e, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_rol(system, addr, data)
            },
            {
                "ROL absolute x",
                opcode => 0x3e, non_destructive => false, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_rol(system, addr, data)
            },
            /**************** ROR ****************/
            {
                "ROR accumulator",
                opcode => 0x6a, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_ror_a()
            },
            {
                "ROR zero page",
                opcode => 0x66, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_zero_page(system, self.pc),
                |addr, data| self.inst_ror(system, addr, data)
            },
            {
                "ROR zero page x",
                opcode => 0x76, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, data| self.inst_ror(system, addr, data)
            },
            {
                "ROR absolute",
                opcode => 0x6e, non_destructive => false, pc_incr => 2, cycle => 6, 
                || self.addressing_absolute(system, self.pc),
                |addr, data| self.inst_ror(system, addr, data)
            },
            {
                "ROR absolute x",
                opcode => 0x7e, non_destructive => false, pc_incr => 2, cycle => 7, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, data| self.inst_ror(system, addr, data)
            },
            /**************** RTI ****************/
            {
                "RTI implied",
                opcode => 0x40, non_destructive => false, pc_incr => 0, cycle => 6, 
                || (0, 0),
                |_addr, _data| self.inst_rti(system)
            },
            /**************** RTS ****************/
            {
                "RTS implied",
                opcode => 0x60, non_destructive => false, pc_incr => 0, cycle => 6, 
                || (0, 0),
                |_addr, _data| self.inst_rti(system)
            },
            /**************** SBC ****************/
            {
                "SBC imm",
                opcode => 0xe9, non_destructive => false, pc_incr => 1, cycle => 2, 
                || self.addressing_immediate(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC zero page", 
                opcode => 0xe5, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC zero page x", 
                opcode => 0xf5, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC absolute", 
                opcode => 0xed, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC absolute x", 
                opcode => 0xfd, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_x(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC absolute y", 
                opcode => 0xf9, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute_y(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC indirect x", 
                opcode => 0xe1, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            {
                "SBC indirect y", 
                opcode => 0xf1, non_destructive => false, pc_incr => 1, cycle => 5, 
                || self.addressing_indirect_y(system, self.pc),
                |_addr, data| self.inst_sbc(data)
            },
            /**************** SEC ****************/
            {
                "SEC implied",
                opcode => 0x38, non_destructive => false, pc_incr => 0, cycle => 6, 
                || (0, 0),
                |_addr, _data| self.inst_sec()
            },
            /**************** SED ****************/
            {
                "SED implied",
                opcode => 0xf8, non_destructive => false, pc_incr => 0, cycle => 6, 
                || (0, 0),
                |_addr, _data| self.inst_sed()
            },
            /**************** SEI ****************/
            {
                "SEI implied",
                opcode => 0x78, non_destructive => false, pc_incr => 0, cycle => 6, 
                || (0, 0),
                |_addr, _data| self.inst_sei()
            },
            /**************** STA ****************/
            {
                "STA zero page", 
                opcode => 0x85, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            {
                "STA zero page x", 
                opcode => 0x95, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            {
                "STA absolute", 
                opcode => 0x8d, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            {
                "STA absolute x", 
                opcode => 0x9d, non_destructive => false, pc_incr => 2, cycle => 5, 
                || self.addressing_absolute_x(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            {
                "STA absolute y", 
                opcode => 0x99, non_destructive => false, pc_incr => 2, cycle => 5, 
                || self.addressing_absolute_y(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            {
                "STA indirect x", 
                opcode => 0x81, non_destructive => false, pc_incr => 1, cycle => 6, 
                || self.addressing_indirect_x(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            {
                "STA indirect y", 
                opcode => 0x91, non_destructive => false, pc_incr => 1, cycle => 6,
                || self.addressing_indirect_y(system, self.pc),
                |addr, _data| self.inst_sta(system, addr)
            },
            /**************** STX ****************/
            {
                "STX zero page", 
                opcode => 0x86, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |addr, _data| self.inst_stx(system, addr)
            },
            {
                "STX zero page y", 
                opcode => 0x96, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_y(system, self.pc),
                |addr, _data| self.inst_stx(system, addr)
            },
            {
                "STX absolute", 
                opcode => 0x8e, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |addr, _data| self.inst_stx(system, addr)
            },
            /**************** STY ****************/
            {
                "STY zero page", 
                opcode => 0x84, non_destructive => false, pc_incr => 1, cycle => 3, 
                || self.addressing_zero_page(system, self.pc),
                |addr, _data| self.inst_sty(system, addr)
            },
            {
                "STY zero page x", 
                opcode => 0x94, non_destructive => false, pc_incr => 1, cycle => 4, 
                || self.addressing_zero_page_x(system, self.pc),
                |addr, _data| self.inst_sty(system, addr)
            },
            {
                "STY absolute", 
                opcode => 0x8c, non_destructive => false, pc_incr => 2, cycle => 4, 
                || self.addressing_absolute(system, self.pc),
                |addr, _data| self.inst_sty(system, addr)
            },
            /**************** TAX ****************/
            {
                "TAX implied",
                opcode => 0xaa, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_tax()
            },
            /**************** TAY ****************/
            {
                "TAY implied",
                opcode => 0xa8, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_tay()
            },
            /**************** TSX ****************/
            {
                "TSX implied",
                opcode => 0xba, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_tsx()
            },
            /**************** TXA ****************/
            {
                "TXA implied",
                opcode => 0x8a, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_txa()
            },
            /**************** TXS ****************/
            {
                "TXS implied",
                opcode => 0x9a, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_txs()
            },
            /**************** TYA ****************/
            {
                "TYA implied",
                opcode => 0x98, non_destructive => false, pc_incr => 0, cycle => 2, 
                || (0, 0),
                |_addr, _data| self.inst_tya()
            }
        )
    }

}

