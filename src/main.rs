use rust_nes_emulator::nes;
extern crate rust_nes_emulator;

use nes::*;
use nes::interface::*;

/* for desktop simulation driver */
// TODO: ある程度完成したらDesktopで動かす部分は別リポジトリに移動して、本リポジトリをライブラリとして参照する
/* for desktop simulation driver */

fn main() {
    let mut sys = System {
        cpu: Cpu {
            a: 0, x: 0, y: 0, pc: 0, sp: 0, p: 0,
        },
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

    // test
    sys.eram.store(|addr, data| {
        println!("addr:{:x}, data:{:x}", addr, data);
    });
    println!("Hello, world!");
}
