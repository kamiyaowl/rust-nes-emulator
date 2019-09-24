use super::cpu::*;

#[derive(Copy, Clone)]
pub enum PulseDutyCycle {
    Duty12_5,
    Duty25_0,
    Duty50_0,
    Duty75_0,
}

#[derive(Copy, Clone)]
pub struct PulseSound {
    // $4000 / $4004
    /// Duty Cycleを設定する
    pub duty_cycle: PulseDutyCycle,
    /// 再生時間カウンタ有効
    pub is_length_counter_halt: bool,
    /// 音響選択
    pub is_constant_volume: bool,
    /// 音量値 4bit
    pub volume: u8,

    // $4001 / $4005
    /// 周波数スイープ有効
    pub is_sweep_enable: bool,
    /// 周波数スイープ値
    pub sweep_period: u8,
    /// 周波数スイープの方向(add/sub)
    pub is_sweep_negative: bool,
    /// 周波数範囲
    pub sweep_shift: u8,

    // $4002, $4003 / $4006, $4007
    /// 周波数
    pub timer_value: u16,
    /// 再生時間
    pub length_counter_load: u8,
}
impl Default for PulseSound {
    fn default() -> Self {
        Self {
            duty_cycle: PulseDutyCycle::Duty12_5,
            is_length_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            is_sweep_enable: false,
            sweep_period: 0,
            is_sweep_negative: false,
            sweep_shift: 0,
            timer_value: 0,
            length_counter_load: 0,
        }
    }
}

impl PulseSound {
    pub fn get_freq(&self) -> u32 {
        CPU_FREQ / (16 * (u32::from(self.timer_value) + 1))
    }
}

#[derive(Copy, Clone)]
pub struct TriangleSound {
    // $4008
    /// 再生時間カウンタ有効
    pub is_length_counter_halt: bool,
    /// 再生時間カウンタ値
    pub counter_load: u8,
    // $400a, $400b
    /// 周波数
    pub timer_value: u16,
    /// 再生時間
    pub length_counter_load: u8,
}

impl Default for TriangleSound {
    fn default() -> Self {
        Self {
            is_length_counter_halt: false,
            counter_load: 0,
            timer_value: 0,
            length_counter_load: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct NoiseSound {
    // $400c
    /// 再生時間カウンタ有効
    pub is_length_counter_halt: bool,
    /// 音響選択
    pub is_constant_volume: bool,
    /// 音量値 4bit
    pub volume: u8,
    // $400E
    /// ノイズの種類
    pub is_noise_type_loop: bool,
    /// 再生時間カウンタ値
    pub noise_period: u8,
    // $400f
    /// 再生時間
    pub length_counter_load: u8,
}

impl Default for NoiseSound {
    fn default() -> Self {
        Self {
            is_length_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            is_noise_type_loop: false,
            noise_period: 0,
            length_counter_load: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DmcSound {
    // $4010
    /// Loop時に割り込み有効
    pub is_irq_enable: bool,
    /// ループ有効
    pub is_loop_enable: bool,
    /// サンプルレートビット 4bit
    pub frequency: u8,
    // $4011
    /// 再生時間
    pub load_counter: u8,
    // $4012
    /// 読み込み先アドレス
    /// $C000-FFFFを参照するので 11AAAAAA-AA000000
    pub sample_addr: u8,
    // $4013
    /// ループに使うデータ量
    /// 0000LLLL, LLLL0001
    pub sample_length: u8,
}

impl Default for DmcSound {
    fn default() -> Self {
        Self {
            is_irq_enable: false,
            is_loop_enable: false,
            frequency: 0,
            load_counter: 0,
            sample_addr: 0,
            sample_length: 0,
        }
    }
}

#[derive(Clone)]
pub struct Apu {
    /// Frame Sequencer、CPUサイクルに連動して加算 11bit
    pub frame_seq_counter: u16,
}
impl Default for Apu {
    fn default() -> Self {
        Self {
            frame_seq_counter: 0,
        }
    }
}

impl Apu {
    /// FrameSeq
    /// TODO: 実装
    #[allow(dead_code)]
    fn increment_seq(&mut self, cpu_cyc: u8) {
        self.frame_seq_counter = (self.frame_seq_counter + u16::from(cpu_cyc)) & 0x03ff; // 11bit
    }
    /// APUの処理を進めます
    pub fn step(&mut self, _cpu: &mut Cpu, _cpu_cyc: u8) {
        // TODO:がんばる

    }
}
