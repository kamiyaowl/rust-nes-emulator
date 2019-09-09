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
/// 構造体のコピーが気になるので、再生が無効化されていたら最初からNoneを返させる
impl System {
    /// 矩形波の設定を取得します
    /// `index` - 0 or 1
    pub fn read_apu_pulse_config(&self, index: u8) -> Option<PulseSound> {
        let mut dst = PulseSound::default();
        debug_assert!(index < 2);
        // 再生無効だったら即返す
        if !self.read_apu_is_enable_pulse(index) {
            return None;
        }
        // pulse1/2でベースアドレス切りかえ
        let base_offset = if index == 0 { APU_PULSE_1_OFFSET } else { APU_PULSE_2_OFFSET };
        // 順番に読んで値を決めるだけ
        dst.dutyCycle = match (self.io_reg[base_offset + 0] >> 6) & 0x03 {
            0 => 875, // 87.5%
            1 => 750, // 75.0%
            2 => 500, // 50.0%
            3 => 250, // 25.0%
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

        Some(dst)
    }

    /// 三角波の設定を取得します
    pub fn read_apu_tri_config(&self) -> Option<TriangleSound> {
        // 再生無効だったら即返す
        if !self.read_apu_is_enable_tri() {
            return None;
        }

        let mut dst = TriangleSound::default();
        // $4008 CRRRRRRRR
        dst.isLengthCounterHalt = (self.io_reg[APU_TRIANGLE_OFFSET + 0] & 0x80) == 0x80;
        dst.counterLoad         =  self.io_reg[APU_TRIANGLE_OFFSET + 0] & 0x7f;
        // $400a TTTTTTTT(timer lower)
        // $400b LLLLLTTT(timer upper)
        dst.timerValue          = u16::from(self.io_reg[APU_TRIANGLE_OFFSET + 2]) |
                                 (u16::from(self.io_reg[APU_TRIANGLE_OFFSET + 3] & 0x07) << 8);
        dst.lengthCounterLoad   = (self.io_reg[APU_TRIANGLE_OFFSET + 3] & 0xf8) >> 3;

        Some(dst)
    }
    
    /// ノイズ波の設定を取得します
    pub fn read_apu_noise_config(&self) -> Option<NoiseSound> {
        // 再生無効だったら即返す
        if !self.read_apu_is_enable_noise() {
            return None;
        }
        let mut dst = NoiseSound::default();
        // $400c --LCVVVV
        dst.isLengthCounterHalt = (self.io_reg[APU_NOISE_OFFSET + 0] & 0x20) == 0x20;
        dst.isConstantVolume    = (self.io_reg[APU_NOISE_OFFSET + 0] & 0x10) == 0x10;
        dst.volume              =  self.io_reg[APU_NOISE_OFFSET + 0] & 0x0f;
        // $400E L---PPPP
        dst.isNoiseTypeLoop     = (self.io_reg[APU_NOISE_OFFSET + 2] & 0x80) == 0x80;
        dst.noisePeriod         =  self.io_reg[APU_NOISE_OFFSET + 2] & 0x0f;
        // $400F LLLLL---
        dst.lengthCounterLoad   = (self.io_reg[APU_NOISE_OFFSET + 3] & 0xf8) >> 3;

        Some(dst)
    }

    /// DMCの設定を取得します
    pub fn read_apu_dmc_config(&self) -> Option<DmcSound> {
        // 再生無効だったら即返す
        if !self.read_apu_is_enable_dmc() {
            return None;
        }
        let mut dst = DmcSound::default();
        // $4010 IL--RRRR
        dst.isIrqEnable  = (self.io_reg[APU_DMC_OFFSET + 0] & 0x80) == 0x80;
        dst.isLoopEnable = (self.io_reg[APU_DMC_OFFSET + 0] & 0x40) == 0x40;
        dst.frequency    =  self.io_reg[APU_DMC_OFFSET + 0] & 0x0f;
        // $4011 -DDDDDDD
        dst.loadCounter  =  self.io_reg[APU_DMC_OFFSET + 1] & 0x7f;
        // $4012 Sample Address
        dst.loadCounter  =  self.io_reg[APU_DMC_OFFSET + 2];
        // $4013 Sample Length
        dst.loadCounter  =  self.io_reg[APU_DMC_OFFSET + 3];

        Some(dst)
    }

    // $4015 Status
    // ---DNTPP
    pub fn read_apu_is_enable_dmc(&self) -> bool {
        (self.io_reg[APU_STATUS_OFFSET] & 0x10) == 0x10
    }
    pub fn read_apu_is_enable_noise(&self) -> bool {
        (self.io_reg[APU_STATUS_OFFSET] & 0x08) == 0x08
    }
    pub fn read_apu_is_enable_tri(&self) -> bool {
        (self.io_reg[APU_STATUS_OFFSET] & 0x04) == 0x04
    }
    pub fn read_apu_is_enable_pulse(&self, index: u8) -> bool {
        debug_assert!(index < 2);
        if index == 0 {
            (self.io_reg[APU_STATUS_OFFSET] & 0x01) == 0x01
        } else {
            (self.io_reg[APU_STATUS_OFFSET] & 0x02) == 0x02
        }
    }

    // TODO: $4017 Frame Counter
}
