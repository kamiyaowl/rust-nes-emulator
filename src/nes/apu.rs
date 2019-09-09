use super::system::*;
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
    pub dutyCycle: PulseDutyCycle,
    /// 再生時間カウンタ有効
    pub isLengthCounterHalt: bool,
    /// 音響選択
    pub isConstantVolume: bool,
    /// 音量値 4bit
    pub volume: u8,

    // $4001 / $4005
    /// 周波数スイープ有効
    pub isSweepEnable: bool,
    /// 周波数スイープ値
    pub sweepPeriod: u8,
    /// 周波数スイープの方向(add/sub)
    pub isSweepNegative: bool,
    /// 周波数範囲
    pub sweepShift: u8,

    // $4002, $4003 / $4006, $4007
    /// 周波数
    pub timerValue: u16,
    /// 再生時間
    pub lengthCounterLoad: u8,
}
impl Default for PulseSound {
    fn default() -> Self {
        Self {
            dutyCycle: PulseDutyCycle::Duty12_5,
            isLengthCounterHalt: false,
            isConstantVolume: false,
            volume: 0,
            isSweepEnable: false,
            sweepPeriod: 0,
            isSweepNegative: false,
            sweepShift: 0,
            timerValue: 0,
            lengthCounterLoad: 0,
        }
    }
}

impl PulseSound {
    pub fn get_freq(&self) -> u32 {
        CPU_FREQ / (16 * (u32::from(self.timerValue) + 1))
    }
}

#[derive(Copy, Clone)]
pub struct TriangleSound {
    // $4008
    /// 再生時間カウンタ有効
    pub isLengthCounterHalt: bool,
    /// 再生時間カウンタ値
    pub counterLoad: u8,
    // $400a, $400b
    /// 周波数
    pub timerValue: u16,
    /// 再生時間
    pub lengthCounterLoad: u8,
}

impl Default for TriangleSound {
    fn default() -> Self {
        Self {
            isLengthCounterHalt: false,
            counterLoad: 0,
            timerValue: 0,
            lengthCounterLoad: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct NoiseSound {
    // $400c
    /// 再生時間カウンタ有効
    pub isLengthCounterHalt: bool,
    /// 音響選択
    pub isConstantVolume: bool,
    /// 音量値 4bit
    pub volume: u8,
    // $400E
    /// ノイズの種類
    pub isNoiseTypeLoop: bool,
    /// 再生時間カウンタ値
    pub noisePeriod: u8,
    // $400f
    /// 再生時間
    pub lengthCounterLoad: u8,
}

impl Default for NoiseSound {
    fn default() -> Self {
        Self {
            isLengthCounterHalt: false,
            isConstantVolume: false,
            volume: 0,
            isNoiseTypeLoop: false,
            noisePeriod: 0,
            lengthCounterLoad: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DmcSound {
    // $4010
    /// Loop時に割り込み有効
    pub isIrqEnable: bool,
    /// ループ有効
    pub isLoopEnable: bool,
    /// サンプルレートビット 4bit
    pub frequency: u8,
    // $4011
    /// 再生時間
    pub loadCounter: u8,
    // $4012
    /// 読み込み先アドレス
    /// $C000-FFFFを参照するので 11AAAAAA-AA000000
    pub sampleAddress: u8,
    // $4013
    /// ループに使うデータ量
    /// 0000LLLL, LLLL0001
    pub sampleLength: u8,
}

impl Default for DmcSound {
    fn default() -> Self {
        Self {
            isIrqEnable: false,
            isLoopEnable: false,
            frequency: 0,
            loadCounter: 0,
            sampleAddress: 0,
            sampleLength: 0,
        }
    }
}


pub struct Apu {
    /// Frame Sequencer、CPUサイクルに連動して加算 11bit
    pub frameSeqCounter: u16,

}
impl Default for Apu {
    fn default() -> Self {
        Self {
            frameSeqCounter: 0,
        }
    }
}

impl Apu {
    /// FrameSe
    fn increment_seq(&mut self, cpu_cyc: u8) {
        self.frameSeqCounter = (self.frameSeqCounter + u16::from(cpu_cyc)) & 0x03ff; // 11bit
    }
    /// APUの処理を進めます
    pub fn step(&mut self, cpu: &mut Cpu, cpu_cyc: u8) {
        // TODO:がんばる

    }
}