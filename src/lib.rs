#![crate_type = "lib"]
#![crate_name = "rust_nes_emulator"]
#![cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
pub mod interface;

pub mod apu;
pub mod cassette;
pub mod cpu;
pub mod cpu_instruction;
pub mod cpu_register;
pub mod pad;
pub mod ppu;
pub mod prelude;
pub mod system;
pub mod system_apu_reg;
pub mod system_ppu_reg;
pub mod video_system;
