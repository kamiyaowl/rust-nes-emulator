extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate rust_nes_emulator;
use rust_nes_emulator::*;
use rust_nes_emulator::interface::*;

#[wasm_bindgen]
pub fn get_screen_width() -> usize {
    VISIBLE_SCREEN_WIDTH
}
#[wasm_bindgen]
pub fn get_screen_height() -> usize {
    VISIBLE_SCREEN_HEIGHT
}
#[wasm_bindgen]
pub fn get_num_of_colors() -> usize {
    NUM_OF_COLOR
}

#[wasm_bindgen]
pub struct WasmEmulator {
    fb: [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    cpu: Cpu,
    cpu_sys: System,
    ppu: Ppu,
    video_sys: VideoSystem,
}
impl Default for WasmEmulator {
    fn default() -> Self {
        Self {
            fb: [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
            cpu: Cpu::default(),
            cpu_sys: System::default(),
            ppu: Ppu::default(),
            video_sys: VideoSystem::default(),
        }
    }
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEmulator {
        WasmEmulator::default()
    }
    /// fbのポインタを取得します
    pub fn get_fb_ptr(&self) -> *const [[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH] {
        self.fb.as_ptr()
    }
    /// 1get_fb_ptr`で得られる配列のサイズを返します
    pub fn get_fb_size(&self) -> usize {
        NUM_OF_COLOR * VISIBLE_SCREEN_WIDTH * VISIBLE_SCREEN_HEIGHT
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
    /// 描画領域1面分更新します
    pub fn step_line(&mut self) {
        let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
        let mut total_cycle: usize = 0;
        while total_cycle < cycle_for_draw_once {
            let cpu_cycle = usize::from(self.cpu.step(&mut self.cpu_sys));
            self.ppu.step(cpu_cycle, &mut self.cpu, &mut self.cpu_sys, &mut self.video_sys, &mut self.fb);
            total_cycle = total_cycle + cpu_cycle;
            // TODO: apu対応(1面分更新だとタイミング的に厳しいかも #8)
        }
    }
}
