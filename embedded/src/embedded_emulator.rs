extern crate rust_nes_emulator;
use rust_nes_emulator::prelude::*;

#[derive(PartialEq,Eq,Copy,Clone,Debug)]
pub enum KeyEvent {
    PressA,
    PressB,
    PressSelect,
    PressStart,
    PressUp,
    PressDown,
    PressLeft,
    PressRight,
    ReleaseA,
    ReleaseB,
    ReleaseSelect,
    ReleaseStart,
    ReleaseUp,
    ReleaseDown,
    ReleaseLeft,
    ReleaseRight,
}

pub struct EmbeddedEmulator {
    pub cpu: Cpu,
    pub cpu_sys: System,
    pub ppu: Ppu,
    pub video_sys: VideoSystem,
}

impl Default for EmbeddedEmulator {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            cpu_sys: System::default(),
            ppu: Ppu::default(),
            video_sys: VideoSystem::default(),
        }
    }
}

impl EmbeddedEmulator {
    pub fn new() -> EmbeddedEmulator {
        EmbeddedEmulator::default()
    }
    /// エミュレータをリセットします
    /// カセットの中身はリセットしないので実機のリセット相当の処理です
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.cpu_sys.reset();
        self.ppu.reset();
        self.video_sys.reset();
        self.cpu.interrupt(&mut self.cpu_sys, Interrupt::RESET);
    }
    /// .nesファイルを読み込みます
    /// `data` - nesファイルのバイナリ
    pub fn load(&mut self, binary: &[u8]) -> bool {
        let success = self
            .cpu_sys
            .cassette
            .from_ines_binary(|addr: usize| binary[addr]);
        if success {
            self.reset();
        }
        success
    }
    /// 描画領域1面分更新します
    pub fn update_frame(&mut self, fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]) {
        let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
        let mut total_cycle: usize = 0;
        while total_cycle < cycle_for_draw_once {
            let cpu_cycle = usize::from(self.cpu.step(&mut self.cpu_sys));
            self.ppu.step(
                cpu_cycle,
                &mut self.cpu,
                &mut self.cpu_sys,
                &mut self.video_sys,
                fb,
            );
            total_cycle = total_cycle + cpu_cycle;
        }
    }
    /// キー入力します
    pub fn update_key(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::PressA      => self.cpu_sys.pad1.push_button(PadButton::A),
            KeyEvent::PressB      => self.cpu_sys.pad1.push_button(PadButton::B),
            KeyEvent::PressSelect => self.cpu_sys.pad1.push_button(PadButton::Select),
            KeyEvent::PressStart  => self.cpu_sys.pad1.push_button(PadButton::Start),
            KeyEvent::PressUp     => self.cpu_sys.pad1.push_button(PadButton::Up),
            KeyEvent::PressDown   => self.cpu_sys.pad1.push_button(PadButton::Down),
            KeyEvent::PressLeft   => self.cpu_sys.pad1.push_button(PadButton::Left),
            KeyEvent::PressRight  => self.cpu_sys.pad1.push_button(PadButton::Right),

            KeyEvent::ReleaseA      => self.cpu_sys.pad1.release_button(PadButton::A),
            KeyEvent::ReleaseB      => self.cpu_sys.pad1.release_button(PadButton::B),
            KeyEvent::ReleaseSelect => self.cpu_sys.pad1.release_button(PadButton::Select),
            KeyEvent::ReleaseStart  => self.cpu_sys.pad1.release_button(PadButton::Start),
            KeyEvent::ReleaseUp     => self.cpu_sys.pad1.release_button(PadButton::Up),
            KeyEvent::ReleaseDown   => self.cpu_sys.pad1.release_button(PadButton::Down),
            KeyEvent::ReleaseLeft   => self.cpu_sys.pad1.release_button(PadButton::Left),
            KeyEvent::ReleaseRight  => self.cpu_sys.pad1.release_button(PadButton::Right),
        }
    }
}
