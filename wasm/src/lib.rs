extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate rust_nes_emulator;
use rust_nes_emulator::*;
use rust_nes_emulator::interface::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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
        console_log!("WasmEmulator::new()");
        WasmEmulator::default()
    }
    /// fbのポインタを取得します
    pub fn get_fb_ptr(&self) -> *const [[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH] {
        console_log!("WasmEmulator::get_fb_ptr()");
        self.fb.as_ptr()
    }
    /// 1get_fb_ptr`で得られる配列のサイズを返します
    pub fn get_fb_size(&self) -> usize {
        console_log!("WasmEmulator::get_fb_size()");
        NUM_OF_COLOR * VISIBLE_SCREEN_WIDTH * VISIBLE_SCREEN_HEIGHT
    }
    /// エミュレータをリセットします
    /// カセットの中身はリセットしないので実機のリセット相当の処理です
    pub fn reset(&mut self) {
        console_log!("WasmEmulator::reset()");
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
        console_log!("WasmEmulator::load()");
        self.reset();
        self.cpu_sys.cassette.from_ines_binary(|addr: usize| binary[addr])
    }
    /// 描画領域1面分更新します
    pub fn step_line(&mut self) {
        console_log!("WasmEmulator::step_line()");
        let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
        let mut total_cycle: usize = 0;
        while total_cycle < cycle_for_draw_once {
            console_log!("cpu_step");
            let cpu_cycle = usize::from(self.cpu.step(&mut self.cpu_sys));
            self.ppu.step(cpu_cycle, &mut self.cpu, &mut self.cpu_sys, &mut self.video_sys, &mut self.fb);
            total_cycle = total_cycle + cpu_cycle;
            // TODO: apu対応(1面分更新だとタイミング的に厳しいかも #8)
        }
    }
}
