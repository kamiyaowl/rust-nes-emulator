use rust_nes_emulator::nes;
extern crate rust_nes_emulator;

use nes::*;
use nes::interface::*;

use std::fs::File;
use std::io::Read;

fn load_cassette(cassette: &mut Cassette, path: String) -> Result<(), Box<dyn std::error::Error>> {
    // nesファイルの読み込み
    let mut file = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    // casseteに展開
    if !cassette.from_ines_binary(|addr: usize| buf[addr]) {
        panic!("ines binary read error");
    }

    Ok(())
}

fn run_cpu_only(path: String, cpu_steps: usize, validate: impl Fn(&Cpu, &System)) -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut cassette: Cassette = Default::default();

    load_cassette(&mut cassette, path)?;
    cpu_sys.cassette = cassette; // 現在はCopy trait

    cpu.reset();
    cpu_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut cpu_cycle: usize = 0;
    for i in 0..cpu_steps {
        println!("================ cpu_step:{}, cpu_cycle:{} ================", i, cpu_cycle);
        cpu_cycle = cpu_cycle + usize::from(cpu.step(&mut cpu_sys));
    }
    validate(&cpu, &cpu_sys);

    Ok(())
}

fn run_cpu_ppu(path: String, validate: impl Fn(&Cpu, &System, &[[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT])) -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();
    let mut cassette: Cassette = Default::default();

    load_cassette(&mut cassette, path)?;
    cpu_sys.cassette = cassette; // 現在はCopy trait

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut fb = [[Color(0,0,0); VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

    let cycle_for_draw_once = usize::from(RENDER_SCREEN_HEIGHT + 1);
    for i in 0..cycle_for_draw_once {
        ppu.step(&mut cpu, &mut cpu_sys, &mut video_sys, |pos, color|{
            fb[pos.1 as usize][pos.0 as usize] = color;
        });
        let mut cpu_cycle = 0;
        while cpu_cycle < CPU_CYCLE_PER_LINE {
            cpu_cycle = cpu_cycle + usize::from(cpu.step(&mut cpu_sys));
        }
    }
    // test draw
    println!("=========================== frame buffer print ===========================");
    for j in 0..VISIBLE_SCREEN_HEIGHT {
        print!("{:02x}:", j);
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let c = fb[j][i];
            if c.0 == 0 && c.1 == 0 && c.2 == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }
    validate(&cpu, &cpu_sys, &fb);

    Ok(())
}
#[test]
fn run_hello_cpu() -> Result<(), Box<dyn std::error::Error>>  {
    run_cpu_only("roms/other/hello.nes".to_string(), 175, |cpu, _sys| {
        // 170step以降はJMPで無限ループしているはず
        assert_eq!(0x804e, cpu.pc);
        assert_eq!(0x01ff, cpu.sp);
        assert_eq!(0x1e,   cpu.a);
        assert_eq!(0x0d,   cpu.x);
        assert_eq!(0x00,   cpu.y);
        assert_eq!(0x34,   cpu.p);
    })
}

#[test]
fn run_hello_ppu() -> Result<(), Box<dyn std::error::Error>>  {
    run_cpu_ppu("roms/other/hello.nes".to_string(), |cpu, _sys, fb| {
    })
}

fn main() {
    run_cpu_ppu("roms/other/hello.nes".to_string(), |cpu, _sys, fb| {
    });
}
