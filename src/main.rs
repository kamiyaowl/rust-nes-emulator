use rust_nes_emulator::nes;
extern crate rust_nes_emulator;

use nes::*;
use nes::interface::*;

use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<std::error::Error>>  {
    let mut cpu = Cpu {
        a: 0, x: 0, y: 0, pc: 0, sp: 0, p: 0, 
    };
    let mut sys = System {
        vram: VideoRam {
            ram: [0; vram::SIZE]
        },
        wram: WorkRam {
            ram: [0; wram::SIZE]
        },
        erom: ExtendedRom {
            rom: [0; erom::SIZE]
        },
        eram: ExtendedRam {
            ram: [0; eram::SIZE]
        },
        prom: ProgramRom {
            rom: [0; prom::SIZE]
        },
    };
    sys.reset();

    /* for desktop simulation driver */
    // nesファイルの読み込み
    let mut file = File::open("roms/other/hello.nes")?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    // system memoryに展開
    let load_result = sys.from_ines_binary(|addr: usize| buf[addr]);
    println!("binary size:{} result:{}", buf.len(), load_result);

    cpu.reset();
    let _cycles = cpu.step(&mut sys);

    Ok(())
}
