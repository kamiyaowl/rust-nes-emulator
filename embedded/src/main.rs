#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m;
use cortex_m_rt;
use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use stm32f7::stm32f7x9;

extern crate rust_nes_emulator;
use rust_nes_emulator::prelude::*;

mod embedded_emulator;
use embedded_emulator::*;

static mut FRAME_BUFFER: [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT] =
    [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

#[allow(dead_code)]
fn print_cpu_info(emu: &EmbeddedEmulator) {
    hprintln!("cpu.pc:{:04x}", emu.cpu.pc).unwrap();
    hprintln!("cpu.sp:{:04x}", emu.cpu.sp).unwrap();
    hprintln!("cpu.a:{:02x}",  emu.cpu.a ).unwrap();
    hprintln!("cpu.x:{:02x}",  emu.cpu.x ).unwrap();
    hprintln!("cpu.y:{:02x}",  emu.cpu.y ).unwrap();
    hprintln!("cpu.p:{:02x}",  emu.cpu.p ).unwrap();
}


#[entry]
fn main() -> ! {
    hprintln!("### rust-nes-emulator-embedded ###").unwrap();

    let peripherals = stm32f7x9::Peripherals::take().unwrap();
    cortex_m::interrupt::free(|_cs| {
        // LD0: RED   : PJ13
        // LD1: GREEN : PJ5
        // LD2: GREEN : PA12

        // GPIO Clock(GPIOJ)
        peripherals
            .RCC
            .ahb1enr
            .write(|w| unsafe { w.bits(0x1 << 9) });
        // Mode Reg: ...Mode1[1:0], Mode0[1:0]
        // input:00, output:01, alt func:10, analog:11
        peripherals
            .GPIOJ
            .moder
            .write(|w| unsafe { w.bits((0x1 << (13 * 2)) | (0x1 << (5 * 2))) });
        // Output Data Reg: ODR15[0]...ODR1[0], ODR0[0]
    });
    hprintln!("peripherals initialize.").unwrap();


    let mut emu = EmbeddedEmulator::new();
    // SDカードが読めるまでは... #74, #75
    let rom = include_bytes!("../../roms/other/hello.nes");
    if !emu.load(rom) {
        loop {
            hprintln!("rom read error").unwrap();
        }
    }
    hprintln!("read ines binary success.").unwrap();

    // emulator test
    print_cpu_info(&emu);
    unsafe { emu.update_frame(&mut FRAME_BUFFER); }
    print_cpu_info(&emu);

    let mut counter: u32 = 0;
    loop {
        hprintln!("### rust-nes-emulator-embedded {} ###", counter).unwrap();

        counter = counter + 1;
        peripherals
            .GPIOJ
            .odr
            .write(|w| unsafe { w.bits((0x1 << 13) | (0x1 << 5)) });
        peripherals.GPIOJ.odr.write(|w| unsafe { w.bits(0) });
    }
}
