#[macro_use]
extern crate rust_nes_emulator;

use rust_nes_emulator::nes;
use nes::*;
use nes::interface::*;
use nes::debugger::*;

// for read ines file
use std::fs::File;
use std::io::Read;

// for save screenshot
extern crate bmp;
use bmp::{Image, Pixel};

// for gui
extern crate piston_window;
use piston_window::*;
extern crate image as im;
use std::time::Instant;

/// NESファイルを読み込んでカセットにロードさせます
fn load_cassette(cassette: &mut Cassette, path: String) -> Result<(), Box<dyn std::error::Error>> {
    debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("read ines from {}", path));
    
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
fn print_framebuffer(fb: &[[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]) {
    println!("=========================== frame buffer print ===========================");
    for j in 0..VISIBLE_SCREEN_HEIGHT {
        print!("{:02x}:", j);
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let c = fb[j][i];
            if c[0] == 0 && c[1] == 0 && c[2] == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

/// FrameBufferの中身をbmpファイルに保存します
fn save_framebuffer(fb: &[[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT], path: String) {
    let mut img = Image::new(VISIBLE_SCREEN_WIDTH as u32, VISIBLE_SCREEN_HEIGHT as u32);

    for j in 0..VISIBLE_SCREEN_HEIGHT {
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let x = i as u32;
            let y = j as u32;
            let c = fb[j][i];
            img.set_pixel(x, y, Pixel::new(c[0], c[1], c[2]));
        }
    }
    debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("save framebuffer to {}", path));
    let _ = img.save(path);
}

/// FrameBufferの中身を保存されたbmpファイルと比較します
fn validate_framebuffer(fb: &[[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT], path: String) -> Result<(), Box<dyn std::error::Error>>  {
    let img = bmp::open(path)?;

    for j in 0..VISIBLE_SCREEN_HEIGHT {
        for i in 0..VISIBLE_SCREEN_WIDTH {
            let x = i as u32;
            let y = j as u32;
            let c = fb[j][i];
            let expect = img.get_pixel(x, y);

            assert_eq!(expect.r, c[0]);
            assert_eq!(expect.g, c[1]);
            assert_eq!(expect.b, c[2]);
        }
    }

    Ok(())
}

fn run_cpu_only(rom_path: String, cpu_steps: usize, validate: impl Fn(&Cpu, &System)) -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    
    load_cassette(&mut cpu_sys.cassette, rom_path)?;

    cpu.reset();
    cpu_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut cpu_cycle: usize = 0;
    for i in 0..cpu_steps {
        debugger_print!(PrintLevel::DEBUG, PrintFrom::TEST, format!("================ cpu_step:{}, cpu_cycle:{} ================", i, cpu_cycle));
        cpu_cycle = cpu_cycle + usize::from(cpu.step(&mut cpu_sys));
    }
    validate(&cpu, &cpu_sys);

    Ok(())
}

fn run_cpu_ppu(rom_path: String, save_path: String, frame_count: usize, validate: impl Fn(&Cpu, &System, &[[[u8; 3]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT])) -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    load_cassette(&mut cpu_sys.cassette, rom_path)?;

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

    // cpuを基準にppuを動かしてあげる
    let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
    for i in 0..frame_count {
        debugger_print!(PrintLevel::DEBUG, PrintFrom::TEST, format!("===================== frame:{} =====================", i));
        
        let mut total_cycle: usize = 0;
        while total_cycle < cycle_for_draw_once {
            let cpu_cycle = usize::from(cpu.step(&mut cpu_sys));
            ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut fb);

            debugger_print!(PrintLevel::DEBUG, PrintFrom::TEST, format!("cycle_for_draw_once={}, total_cycle={}, cpu_cycle={}", cycle_for_draw_once, total_cycle, cpu_cycle));
            total_cycle = total_cycle + cpu_cycle;
        }
    }

    print_framebuffer(&fb);
    save_framebuffer(&fb, save_path);

    validate(&cpu, &cpu_sys, &fb);

    Ok(())
}

#[test]
fn run_hello_cpu() -> Result<(), Box<dyn std::error::Error>>  {
    debugger_enable_fileout!("run_hello_cpu.log".to_string());
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

// #[test]
// fn run_hello_ppu() -> Result<(), Box<dyn std::error::Error>> {
//     debugger_init!();
//     run_cpu_ppu("roms/other/hello.nes".to_string(), "framebuffer_run_hello_ppu.bmp".to_string(), 1, |cpu, _sys, fb| {
//         // 170step以降はJMPで無限ループしているはず
//         assert_eq!(0x804e, cpu.pc);
//         assert_eq!(0x01ff, cpu.sp);
//         assert_eq!(0x1e,   cpu.a);
//         assert_eq!(0x0d,   cpu.x);
//         assert_eq!(0x00,   cpu.y);
//         assert_eq!(0x34,   cpu.p);
//         // FBの結果を精査する
//         let _ = validate_framebuffer(fb, "screenshot/hello.bmp".to_string());
//     })
// }

// #[test]
// fn run_nestest_boot() -> Result<(), Box<dyn std::error::Error>> {
//     debugger_init!();
//     run_cpu_ppu("roms/nes-test-roms/other/nestest.nes".to_string(), "framebuffer_nestest_boot.bmp".to_string(), 3, |_cpu, _sys, _fb| {
//         // FBの結果を精査する
//         // unimplemented!();
//         // let _ = validate_framebuffer(fb, "screenshot/nestest_1.bmp".to_string());
//     })
// }

fn run_gui(rom_path: String) -> Result<(), Box<dyn std::error::Error>> {
    // guiの準備
    let mut window: PistonWindow =
        WindowSettings::new("rust-nes-emulator", [256, 240])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // emulatorの準備
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    load_cassette(&mut cpu_sys.cassette, rom_path)?;

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

    // FPS平均計算用
    const ELAPSED_N: usize = 128;
    let mut elapsed_ptr = 0;
    let mut elapsed_secs = [0.0; ELAPSED_N];

    while let Some(event) = window.next() {
        // 1frame分実行する
        let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT+1);

        let mut total_cycle: usize = 0;
        let start = Instant::now();
        while total_cycle < cycle_for_draw_once {
            let cpu_cycle = usize::from(cpu.step(&mut cpu_sys));
            ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut fb);
            total_cycle = total_cycle + cpu_cycle;

            // debug
            // let mut s = String::new();
            // std::io::stdin().read_line(&mut s).unwrap();
        }
        let end = start.elapsed(); // 実行時間計測
        elapsed_secs[elapsed_ptr] = (end.as_millis() as f32) / 1000.0;
        elapsed_ptr = (elapsed_ptr + 1) % ELAPSED_N;

        // TODO: Key入力でレジスタを叩く
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::J => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("a"));      cpu_sys.pad1.push_button(PadButton::A) },
                Key::K => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("b"));      cpu_sys.pad1.push_button(PadButton::B) },
                Key::U => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("select")); cpu_sys.pad1.push_button(PadButton::Select) },
                Key::I => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("start"));  cpu_sys.pad1.push_button(PadButton::Start) },
                Key::W => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("up"));     cpu_sys.pad1.push_button(PadButton::Up) },
                Key::S => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("down"));   cpu_sys.pad1.push_button(PadButton::Down) },
                Key::A => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("left"));   cpu_sys.pad1.push_button(PadButton::Left) },
                Key::D => { debugger_print!(PrintLevel::INFO, PrintFrom::MAIN, format!("right"));  cpu_sys.pad1.push_button(PadButton::Right) },
                Key::P => { 
                    save_framebuffer(&fb, "run_gui_ss.bmp".to_string());
                 },
                _ => {},
            }
        };

        // 書く
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for j in 0..VISIBLE_SCREEN_HEIGHT {
                for i in 0..VISIBLE_SCREEN_WIDTH {
                    let x = i as u32;
                    let y = j as u32;
                    let color = fb[j][i];
                    rectangle([(color[0] as f32) / 255.0, (color[1] as f32) / 255.0, (color[2] as f32) / 255.0, 1.0],
                                [x as f64, y as f64, (x + 1) as f64, (y + 1) as f64],
                                c.transform, g);
                }
            }
        });
    }

    // おわりにパフォーマンスとか出してみる
    let sum = elapsed_secs.iter().fold(0.0, |sum, a| sum + a);
    let average = sum / (ELAPSED_N as f32);
    let fps = 1.0 / average;
    debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[performance] elapsed_average={}, fps_average={}", average, fps));

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_gui("roms/nes-test-roms/other/nestest.nes".to_string())
}
