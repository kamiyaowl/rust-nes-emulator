use super::system::*;
use super::apu::*;

pub const APU_PULSE_1_OFFSET:        usize = 0x00;
pub const APU_PULSE_2_OFFSET:        usize = 0x04;
pub const APU_TRIANGLE_OFFSET:       usize = 0x08;
pub const APU_NOISE_OFFSET:          usize = 0x0c;
pub const APU_DMC_OFFSET:            usize = 0x10;
pub const APU_STATUS_OFFSET:         usize = 0x15;
pub const APU_FRAMECOUNTER_OFFSET:   usize = 0x15;

/// APU & I/O(PAD) Register Implement
/// APUのみ(DMAはsystem_ppu_reg.rs, padはレジスタの変数を使わない)
/// 固定小数点演算が入るものは、別途関数で計算する(定数を返すだけなら構わない)
impl System {
    /// Puluse波の設定を取得します
    /// `index` - 0 or 1
    pub fn read_apu_pulse_config(&self, index: u8) -> PulseSound {
        let mut dst = PulseSound::default();
        debug_assert!(index < 2);
        let base_offset = if index == 0 { APU_PULSE_1_OFFSET } else { APU_PULSE_2_OFFSET };
        // 順番に読んで値を決めるだけ
        dst.dutyCycle = match (self.io_reg[base_offset + 0] >> 6) & 0x03 {
            0 => 87.5,
            1 => 75.0,
            2 => 50.0,
            3 => 25.0,
            _ => panic!("invalid pulse dutyCycle: {}", self.io_reg[base_offset + 0]),
        };
        // $4000 DDLCVVVV
        dst.isLengthCounterHalt = (self.io_reg[base_offset + 0] & 0x20) == 0x20;
        dst.isConstantVolume    = (self.io_reg[base_offset + 0] & 0x10) == 0x10;
        dst.volume              =  self.io_reg[base_offset + 0] & 0x0f;
        // $4001 EPPPNSSS
        dst.isSweepEnable       = (self.io_reg[base_offset + 1] & 0x80) == 0x80;
        dst.sweepPeriod         = (self.io_reg[base_offset + 1] & 0x70) >> 4;
        dst.isSweepNegative     = (self.io_reg[base_offset + 1] & 0x04) == 0x04;
        dst.sweepShift          =  self.io_reg[base_offset + 1] & 0x07;
        // $4002 TTTTTTTT(timer lower)
        // $4003 LLLLLTTT(timer Upper)
        dst.timerValue          = u16::from(self.io_reg[base_offset + 2]) |
                                 (u16::from(self.io_reg[base_offset + 3] & 0x07) << 8);
        dst.lengthCounterLoad   = (self.io_reg[base_offset + 3] & 0xf8) >> 3;

        dst
    }
    
}
