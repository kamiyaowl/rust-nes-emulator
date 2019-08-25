use rust_nes_emulator::nes;
extern crate rust_nes_emulator;

use nes::*;
use nes::interface::*;

// for read ines file
use std::fs::File;
use std::io::Read;

// for save screenshot
extern crate bmp;
use bmp::{Image, Pixel};

/// NESファイルを読み込んでカセットにロードさせます
fn load_cassette(cassette: &mut Cassette, path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("read ines from {}", path);
    let mut file = File::open(path)?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    // casseteに展開
    if !cassette.from_ines_binary(|addr: usize| buf[addr]) {
        panic!("ines binary read error");
    }

    Ok(())
}

/// FrameBufferの中身をコンソール出力します。色があれば#, なければ.が出力されます
fn print_framebuffer(fb: &[[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]) {
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
}

/// FrameBufferの中身をbmpファイルに保存します
fn save_framebuffer(fb: &[[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT], path: String) {
    let mut img = Image::new(VISIBLE_SCREEN_WIDTH as u32, VISIBLE_SCREEN_HEIGHT as u32);

    for j in 0..VISIBLE_SCREEN_HEIGHT {
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let x = i as u32;
            let y = j as u32;
            let c = fb[j][i];
            img.set_pixel(x, y, Pixel::new(c.0, c.1, c.2));
        }
    }
    println!("save framebuffer to {}", path);
    let _ = img.save(path);
}

/// FrameBufferの中身を保存されたbmpファイルと比較します
fn validate_framebuffer(fb: &[[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT], path: String) -> Result<(), Box<dyn std::error::Error>>  {
    let img = bmp::open(path)?;

    for j in 0..VISIBLE_SCREEN_HEIGHT {
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let x = i as u32;
            let y = j as u32;
            let c = fb[j][i];
            let expect = img.get_pixel(x, y);

            assert_eq!(expect.r, c.0);
            assert_eq!(expect.g, c.1);
            assert_eq!(expect.b, c.2);
        }
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

fn run_cpu_ppu(rom_path: String, save_path: String, validate: impl Fn(&Cpu, &System, &[[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT])) -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();
    let mut cassette: Cassette = Default::default();

    load_cassette(&mut cassette, rom_path)?;
    cpu_sys.cassette = cassette; // 現在はCopy trait

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut fb = [[Color(0,0,0); VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

    // cpuを基準にppuを動かしてあげる
    let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
    let mut total_cycle: usize = 0;
    while total_cycle < cycle_for_draw_once {
        let cpu_cycle = usize::from(cpu.step(&mut cpu_sys));
        ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut cassette, &mut fb);

        // println!("[debug] cycle_for_draw_once={}, total_cycle={}, cpu_cycle={}", cycle_for_draw_once, total_cycle, cpu_cycle);
        total_cycle = total_cycle + cpu_cycle;
    }

    print_framebuffer(&fb);
    save_framebuffer(&fb, save_path);

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
fn run_hello_ppu() -> Result<(), Box<dyn std::error::Error>> {
    run_cpu_ppu("roms/other/hello.nes".to_string(), "framebuffer_run_hello_ppu.bmp".to_string(), |cpu, _sys, fb| {
        // 170step以降はJMPで無限ループしているはず
        assert_eq!(0x804e, cpu.pc);
        assert_eq!(0x01ff, cpu.sp);
        assert_eq!(0x1e,   cpu.a);
        assert_eq!(0x0d,   cpu.x);
        assert_eq!(0x00,   cpu.y);
        assert_eq!(0x34,   cpu.p);
        // FBの結果を精査する
        let _ = validate_framebuffer(fb, "screenshot/hello.bmp".to_string());
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_hello_ppu()
}
