#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger


use cortex_m;
use cortex_m_rt;
use cortex_m_semihosting::{hprintln, hprint};

use cortex_m_rt::entry;
use stm32f7::stm32f7x9;

extern crate rust_nes_emulator;
use rust_nes_emulator::prelude::*;

/// FrameBufferの中身をコンソール出力します。色があれば#, なければ.が出力されます
#[allow(dead_code)]
fn print_framebuffer(fb: &[[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]) {
    hprintln!("=========================== frame buffer print ===========================").unwrap();
    for j in 0..VISIBLE_SCREEN_HEIGHT {
        hprint!("{:02x}:", j).unwrap();
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let c = fb[j][i];
            if c[0] == 0 && c[1] == 0 && c[2] == 0 {
                hprint!(".").unwrap();
            } else {
                hprint!("#").unwrap();
            }
        }
        hprintln!("").unwrap();
    }
}

static mut FRAME_BUFFER: [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT] = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

#[entry]
fn main() -> ! {
    hprintln!("### rust-nes-emulator-embedded ###").unwrap();

    // SDカードが読めるまでは...
    let rom = include_bytes!("../../roms/other/hello.nes");

    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();
    // let mut fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

    let peripherals = stm32f7x9::Peripherals::take().unwrap();

    cortex_m::interrupt::free(|_cs| {
        // LD0: RED   : PJ13
        // LD1: GREEN : PJ5
        // LD2: GREEN : PA12

        // GPIO Clock(GPIOJ)
        peripherals.RCC.ahb1enr.write(|w| unsafe { w.bits(0x1 << 9) });
        // Mode Reg: ...Mode1[1:0], Mode0[1:0]
        // input:00, output:01, alt func:10, analog:11
        peripherals.GPIOJ.moder.write(|w| unsafe { w.bits((0x1 << (13*2)) | (0x1 << ( 5*2))) });
        // Output Data Reg: ODR15[0]...ODR1[0], ODR0[0]
    });

    // if !cpu_sys.cassette.from_ines_binary(|addr: usize| rom[addr]) {
    //     loop {
    //         hprintln!("rom read error").unwrap();
    //     }
    // }

    // let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
    // let frame_count = 2;
    // for _i in 0..frame_count {
    //     let mut total_cycle: usize = 0;
    //     while total_cycle < cycle_for_draw_once {
    //         let cpu_cycle = usize::from(cpu.step(&mut cpu_sys));
    //         ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut fb);

    //         total_cycle = total_cycle + cpu_cycle;
    //     }
    // }
    // print_framebuffer(&fb);

    let mut counter: u32 = 0;
    loop {
        hprintln!("### rust-nes-emulator-embedded {} ###", counter).unwrap();

        counter = counter + 1;
        peripherals.GPIOJ.odr.write(|w| unsafe { w.bits((0x1 << 13) | (0x1 << 5)) });
        peripherals.GPIOJ.odr.write(|w| unsafe { w.bits(0) });
    }
}
