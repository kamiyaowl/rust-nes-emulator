#![no_std]
#![feature(lang_items, core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

extern crate rust_nes_emulator;
use rust_nes_emulator::prelude::*;

pub const EMBEDDED_EMULATOR_NUM_OF_COLOR: usize = 3;
pub const EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH: usize = 256;
pub const EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT: usize = 240;

// 泣く泣くの策、structをそのままcに公開できなかったので
static mut EMULATOR: Option<EmbeddedEmulator> = None;

#[repr(u8)]
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
}

impl Default for EmbeddedEmulator {
    fn default() -> Self {
        Self {
            cpu: Cpu::default(),
            cpu_sys: System::default(),
            ppu: Ppu::default(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_init() {
    EMULATOR = Some(EmbeddedEmulator::default());
}

/// エミュレータをリセットします
/// カセットの中身はリセットしないので実機のリセット相当の処理です
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_reset() {
    if let Some(ref mut emu) = EMULATOR {
        emu.cpu.reset();
        emu.cpu_sys.reset();
        emu.ppu.reset();
        emu.cpu.interrupt(&mut emu.cpu_sys, Interrupt::RESET);
    }
}

/// .nesファイルを読み込みます
/// `data` - nesファイルのバイナリ
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_load() -> bool {
    let binary = include_bytes!("../../roms/other/hello.nes");
    // let binary = include_bytes!("../../roms/my_dump/mario.nes");

    if let Some(ref mut emu) = EMULATOR {
        let success = emu
            .cpu_sys
            .cassette
            .from_ines_binary(|addr: usize| binary[addr]);
        if success {
            EmbeddedEmulator_reset();
        }
        success
    } else {
        false
    }
}

/// 描画領域1面分更新します
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_update_screen(
    fb: &mut [[[u8; EMBEDDED_EMULATOR_NUM_OF_COLOR]; EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH];
             EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT],
) {
    if let Some(ref mut emu) = EMULATOR {
        let mut total_cycle: usize = 0;
        while total_cycle < CYCLE_PER_DRAW_FRAME {
            let cpu_cycle = usize::from(emu.cpu.step(&mut emu.cpu_sys));
            if let Some(interrupt) = emu.ppu.step(cpu_cycle, &mut emu.cpu_sys, fb) {
                emu.cpu.interrupt(&mut emu.cpu_sys, interrupt);
            }
            total_cycle = total_cycle + cpu_cycle;
        }
    }
}

/// キー入力します
#[no_mangle]
pub unsafe extern "C" fn EmbeddedEmulator_update_key(key: KeyEvent) {
    if let Some(ref mut emu) = EMULATOR {
        match key {
            KeyEvent::PressA => emu.cpu_sys.pad1.push_button(PadButton::A),
            KeyEvent::PressB => emu.cpu_sys.pad1.push_button(PadButton::B),
            KeyEvent::PressSelect => emu.cpu_sys.pad1.push_button(PadButton::Select),
            KeyEvent::PressStart => emu.cpu_sys.pad1.push_button(PadButton::Start),
            KeyEvent::PressUp => emu.cpu_sys.pad1.push_button(PadButton::Up),
            KeyEvent::PressDown => emu.cpu_sys.pad1.push_button(PadButton::Down),
            KeyEvent::PressLeft => emu.cpu_sys.pad1.push_button(PadButton::Left),
            KeyEvent::PressRight => emu.cpu_sys.pad1.push_button(PadButton::Right),

            KeyEvent::ReleaseA => emu.cpu_sys.pad1.release_button(PadButton::A),
            KeyEvent::ReleaseB => emu.cpu_sys.pad1.release_button(PadButton::B),
            KeyEvent::ReleaseSelect => emu.cpu_sys.pad1.release_button(PadButton::Select),
            KeyEvent::ReleaseStart => emu.cpu_sys.pad1.release_button(PadButton::Start),
            KeyEvent::ReleaseUp => emu.cpu_sys.pad1.release_button(PadButton::Up),
            KeyEvent::ReleaseDown => emu.cpu_sys.pad1.release_button(PadButton::Down),
            KeyEvent::ReleaseLeft => emu.cpu_sys.pad1.release_button(PadButton::Left),
            KeyEvent::ReleaseRight => emu.cpu_sys.pad1.release_button(PadButton::Right),
        }
    }
}
