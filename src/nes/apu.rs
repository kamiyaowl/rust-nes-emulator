use super::system::*;

#[derive(Copy, Clone)]
pub struct PulseSound {
    // $4000 / $4004
    /// 0-100でDuty Cycleを設定する
    pub dutyCycle: u8,
    /// 再生時間カウンタ有効
    pub lengthCounterHalt: bool,
    /// 音響選択
    pub constantVolume: bool,
    /// 音量値 4bit
    pub volume: u8,

    // $4001 / $4005
    /// 周波数スイープ有効
    pub isSweepEnable: bool,
    /// 周波数スイープ値
    pub sweepPeriod: u8,
    /// 周波数スイープの方向(add/sub)
    pub isSweepPositive: bool,
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
            dutyCycle: 0,
            lengthCounterHalt: false,
            constantVolume: false,
            volume: 0,
            isSweepEnable: false,
            sweepPeriod: 0,
            isSweepPositive: false,
            sweepShift: 0,
            timerValue: 0,
            lengthCounterLoad: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TriangleSound {
    // $4008
    /// 再生時間カウンタ有効
    pub lengthCounterHalt: bool,
    /// 再生時間カウンタ値
    pub counterValue: u8,
    // $400a, $400b
    /// 周波数
    pub timerValue: u16,
    /// 再生時間
    pub lengthCounterLoad: u8,
}

impl Default for TriangleSound {
    fn default() -> Self {
        Self {
            lengthCounterHalt: false,
            counterValue: 0,
            timerValue: 0,
            lengthCounterLoad: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct NoiseSound {
    // $400c
    /// 再生時間カウンタ有効
    pub lengthCounterHalt: bool,
    /// 音響選択
    pub constantVolume: bool,
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
            lengthCounterHalt: false,
            constantVolume: false,
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
    pub irqEnable: bool,
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
            irqEnable: false,
            isLoopEnable: false,
            frequency: 0,
            loadCounter: 0,
            sampleAddress: 0,
            sampleLength: 0,
        }
    }
}


pub struct Apu {
}
impl Default for Apu {
    fn default() -> Self {
        Self {
        }
    }
}

impl Apu {
}