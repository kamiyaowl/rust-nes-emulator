extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate rust_nes_emulator;
use rust_nes_emulator::*;
use rust_nes_emulator::interface::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct WasmEmulator {
    fb: [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    cpu: Cpu,
    cpu_sys: System,
    ppu: Ppu,
    video_sys: VideoSystem,
}

#[wasm_bindgen]
impl WasmEmulator {
    /// fbのポインタを取得します
    pub fn get_fb_ptr(&self) -> *const [[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH] {
        self.fb.as_ptr()
    }
    /// エミュレータをリセットします
    /// カセットの中身はリセットしないので実機のリセット相当の処理です
    pub fn reset(&mut self) {
        self.fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];
        self.cpu.reset();
        self.cpu_sys.reset();
        self.ppu.reset();
        self.video_sys.reset();
        self.cpu.interrupt(&mut self.cpu_sys, Interrupt::RESET);
    }
    /// .nesファイルを読み込みます
    /// `data` - nesファイルのバイナリ
    pub fn load(&mut self, binary: &[u8]) -> bool {
        self.reset();
        self.cpu_sys.cassette.from_ines_binary(|addr: usize| binary[addr])
    }
    /// 描画領域1line分を更新します
    pub fn step_line(&mut self) {
    }
}
