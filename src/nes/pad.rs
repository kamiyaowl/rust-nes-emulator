use super::interface::*;

#[derive(Copy, Clone)]
pub enum Button {
    A,B,Select,Start,Up,Down,Left,Right
}

pub struct Pad {
    /// 現在のボタン入力を示すシフトレジスタ
    pub button_shift_reg: u8,
    /// trueだと入力をロードし続ける
    pub strobe_enable: bool,
}
impl Default for Pad {
    fn default() -> Self {
        Self {
            button_shift_reg: 0,
            strobe_enable: false,
        }
    }
}


impl EmulateControl for Pad {
    fn reset(&mut self) {
        self.button_shift_reg = 0;
        self.strobe_enable = false;
    }
    fn get_dump_size() -> usize {
        0x4
    }
    fn dump(&self, read_callback: impl Fn(usize, u8)) {
        // strobeは保持するけど入力は破棄する
        read_callback(0, if self.strobe_enable { 1 } else { 0 });
        // 0x1~0x3 padding
    }
    fn restore(&mut self, write_callback: impl Fn(usize) -> u8) {
        self.strobe_enable = write_callback(0) == 0x1;
    }
}

impl Pad {
    /// pad入力をクリアして取り込み直します
    /// 0x4016/0x4017に1/0を
    pub fn write_strobe(&mut self, is_enable: bool) {
        self.strobe_enable = is_enable;
    }
    /// pad出力を読み取ります、シフトレジスタになっているため読むたびに状態を破壊します    
    /// xxxx_xMES
    /// M - microphone unimplement
    /// E - Expansion controller unimplement
    /// S - Primary controller
    pub fn read_dout(&mut self) -> u8 {
        let data = self.button_shift_reg & 0x01u8;
        // strobeが無効であればデータを進める
        if !self.strobe_enable {
            self.button_shift_reg = self.button_shift_reg.wrapping_shr(1);
        }
        data
    }
    /// 押されたボタンをシフトレジスタに記録します。この関数はエミュの外側から呼ぶことを想定しています
    pub fn push_button(&mut self, button: Button) {
        if self.strobe_enable {
            match button {
                Button::A      => self.button_shift_reg = self.button_shift_reg | 0x01u8,
                Button::B      => self.button_shift_reg = self.button_shift_reg | 0x02u8,
                Button::Select => self.button_shift_reg = self.button_shift_reg | 0x04u8,
                Button::Start  => self.button_shift_reg = self.button_shift_reg | 0x08u8,
                Button::Up     => self.button_shift_reg = self.button_shift_reg | 0x10u8,
                Button::Down   => self.button_shift_reg = self.button_shift_reg | 0x20u8,
                Button::Left   => self.button_shift_reg = self.button_shift_reg | 0x40u8,
                Button::Right  => self.button_shift_reg = self.button_shift_reg | 0x80u8,
            }
        }
    }
}