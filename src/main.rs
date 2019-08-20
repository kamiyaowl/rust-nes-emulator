use rust_nes_emulator::nes;
extern crate rust_nes_emulator;

use nes::*;
use nes::interface::*;

use std::fs::File;
use std::io::Read;

fn run_image(path: String, cycles: usize, validate: impl Fn(&Cpu, &System)) -> Result<(), Box<dyn std::error::Error>> {
    let mut cassette_emu = Cassette {
        mapper: Mapper::Unknown,
        prg_rom: [0; 0x8000],
        chr_rom: [0; 0x2000],
    };

    // nesファイルの読み込み
    let mut file = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    // casseteに展開
    if !cassette_emu.from_ines_binary(|addr: usize| buf[addr]) {
        panic!("ines binary read error");
    }

    // はじめる
    let mut cpu = Cpu {
        a: 0, x: 0, y: 0, pc: 0, sp: 0, p: 0, 
    };
    let mut sys = System {
        vram:    [0; system::VRAM_SIZE],
        wram:    [0; system::WRAM_SIZE],
        ppu_reg: [0; system::PPU_REG_SIZE],
        io_reg:  [0; system::APU_AND_IO_REG_SIZE],
        cassette: cassette_emu,
    };
    sys.reset();
    cpu.reset();

    cpu.interrupt(&mut sys, Interrupt::RESET);
    for i in 0..cycles {
        println!("================ {} ================", i);
        let _cycles = cpu.step(&mut sys);
    }
    validate(&cpu, &sys);

    Ok(())
}

#[test]
fn run_hello() -> Result<(), Box<dyn std::error::Error>>  {
    run_image("roms/other/hello.nes".to_string(), 175, |cpu, _sys| {
        // 170cyc以降はJMPで無限ループしているはず
        assert_eq!(0x804e, cpu.pc);
        assert_eq!(0x01ff, cpu.sp);
        assert_eq!(0x1e,   cpu.a);
        assert_eq!(0x0d,   cpu.x);
        assert_eq!(0x00,   cpu.y);
        assert_eq!(0x34,   cpu.p);
    })
}

fn main() {
}
