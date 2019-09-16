#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f7::stm32f7x9;

#[entry]
fn main() -> ! {
    let peripherals = stm32f7x9::Peripherals::take().unwrap();

    // LD0: RED   : PJ13
    // LD1: GREEN : PJ5
    // LD2: GREEN : PA12

    // GPIO Clock(GPIOJ)
    peripherals.RCC.ahb1enr.write(|w| unsafe { w.bits(0x1 << 9) });
    // Mode Reg: ...Mode1[1:0], Mode0[1:0]
    // input:00, output:01, alt func:10, analog:11
    peripherals.GPIOJ.moder.write(|w| unsafe { w.bits((0x1 << (13*2)) | (0x1 << ( 5*2))) });
    // Output Data Reg: ODR15[0]...ODR1[0], ODR0[0]
    
    let mut counter: u32 = 0;
    loop {
        counter = counter + 1;
        peripherals.GPIOJ.odr.write(|w| unsafe { w.bits((0x1 << 13) | (0x1 << 5)) });
        peripherals.GPIOJ.odr.write(|w| unsafe { w.bits(0) });
    }
}
