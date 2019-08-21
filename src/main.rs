use rust_nes_emulator::nes;
extern crate rust_nes_emulator;

use nes::*;
use nes::interface::*;

use std::fs::File;
use std::io::Read;

fn run_image(path: String, cpu_steps: usize, validate: impl Fn(&Cpu, &System)) -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu: Cpu = Default::default();
    let mut sys: System = Default::default();
    let mut cassette_emu: Cassette = Default::default();

    // nesファイルの読み込み
    let mut file = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    // casseteに展開
    if !cassette_emu.from_ines_binary(|addr: usize| buf[addr]) {
        panic!("ines binary read error");
    }

    // はじめる
    sys.cassette = cassette_emu; // 現在はCopy trait
    sys.reset();
    cpu.reset();

    cpu.interrupt(&mut sys, Interrupt::RESET);
    let mut cpu_cycle: usize = 0;
    for i in 0..cpu_steps {
        println!("================ cpu_step:{}, cpu_cycle:{} ================", i, cpu_cycle);
        cpu_cycle = cpu_cycle + usize::from(cpu.step(&mut sys));
    }
    validate(&cpu, &sys);

    Ok(())
}

#[test]
fn run_hello() -> Result<(), Box<dyn std::error::Error>>  {
    run_image("roms/other/hello.nes".to_string(), 175, |cpu, _sys| {
        // 170step以降はJMPで無限ループしているはず
        assert_eq!(0x804e, cpu.pc);
        assert_eq!(0x01ff, cpu.sp);
        assert_eq!(0x1e,   cpu.a);
        assert_eq!(0x0d,   cpu.x);
        assert_eq!(0x00,   cpu.y);
        assert_eq!(0x34,   cpu.p);
    })
}

fn main() {
    let _result = run_image("roms/other/hello.nes".to_string(), 175, |_cpu, _sys| {});
}
