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

/// cpuだけで指定したサイクル流す
fn run_cpu_only(rom_path: String, cpu_steps: usize, validate: impl Fn(&Cpu, &System)) -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cpu_cycle: usize = 0;
    for i in 0..cpu_steps {
        debugger_print!(PrintLevel::DEBUG, PrintFrom::TEST, format!("================ cpu_step:{}, cpu_cycle:{} ================", i, cpu_cycle));
        cpu_cycle = cpu_cycle + usize::from(cpu.step(&mut cpu_sys, &ppu));
    }
    validate(&cpu, &cpu_sys);

    Ok(())
}

/// 指定したフレーム数だけ流す
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
            let cpu_cycle = usize::from(cpu.step(&mut cpu_sys, &ppu));
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

/// nestestを起動して、テストを実行してスクショ比較する
fn run_nestest(rom_path: String) -> Result<(), Box<dyn std::error::Error>> {
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
    for i in 0..60 {
        let mut total_cycle: usize = 0;
        while total_cycle < cycle_for_draw_once {
            let cpu_cycle = usize::from(cpu.step(&mut cpu_sys, &ppu));
            ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut fb);

            debugger_print!(PrintLevel::DEBUG, PrintFrom::TEST, format!("cycle_for_draw_once={}, total_cycle={}, cpu_cycle={}", cycle_for_draw_once, total_cycle, cpu_cycle));
            total_cycle = total_cycle + cpu_cycle;
        }
        match i {
            6 => {
                // 起動画像が出るはず
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] validate normal menu screenshot", i));
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_normal_menu.bmp".to_string());
                let _ = validate_framebuffer(&fb, "screenshot/nestest_normal_menu.bmp".to_string());
            },
            7 => {
                // テスト開始ボタン押させる
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] press start button", i));
                cpu_sys.pad1.push_button(PadButton::Start);
            },
            25 => {
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] validate normal test pass screenshot", i));
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_normal.bmp".to_string());
                let _ = validate_framebuffer(&fb, "screenshot/nestest_normal.bmp".to_string());
            },
            26 => {
                // unofficial testに遷移させる
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] press select button", i));
                cpu_sys.pad1.push_button(PadButton::Select);
            },
            29 => {
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] validate extra menu screenshot", i));
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_extra_menu.bmp".to_string());
                let _ = validate_framebuffer(&fb, "screenshot/nestest_extra_menu.bmp".to_string());
            },
            30 => {
                // テスト開始ボタン押させる
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] press start button", i));
                cpu_sys.pad1.push_button(PadButton::Start);
            },
            50 => {
                debugger_print!(PrintLevel::INFO, PrintFrom::TEST, format!("[frame={}] validate extra test pass screenshot", i));
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_extra.bmp".to_string());
                let _ = validate_framebuffer(&fb, "screenshot/nestest_extra.bmp".to_string());
            },
            _ => {},
        };
    }
    debugger_disable_fileout!();

    Ok(())
}

/// hello worldのromで、一通りの処理が終わって無限ループまでたどり着くことを確認する
#[test]
fn test_run_hello_cpu() -> Result<(), Box<dyn std::error::Error>>  {
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

/// 画面上にhello world!が正しく表示されることを確認する
#[test]
fn test_run_hello_ppu() -> Result<(), Box<dyn std::error::Error>> {
    run_cpu_ppu("roms/other/hello.nes".to_string(), "test_run_hello_ppu.bmp".to_string(), 1, |cpu, _sys, fb| {
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

#[test]
fn test_run_nestest() -> Result<(), Box<dyn std::error::Error>> {
    run_nestest("roms/nes-test-roms/other/nestest.nes".to_string())
}


////////////////////////////////////////////////////////////////
/// GUIでグイグイ

extern crate piston_window;
extern crate image as im;

use piston_window::*;
use std::time::Instant;

fn main() {
    // let rom_path = "roms/nes-test-roms/other/nestest.nes".to_string();
    // let rom_path = "roms/nes-test-roms/scrolltest/sssscroll.nes".to_string();
    // let rom_path = "roms/my_dump/mario.nes".to_string();
    let rom_path = "roms/my_dump/donkey.nes".to_string();
    // emu
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    load_cassette(&mut cpu_sys.cassette, rom_path).unwrap();

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];
    let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT);

    // windowの準備
    let scale = 2;
    let width = (VISIBLE_SCREEN_WIDTH * scale) as u32;
    let height = (VISIBLE_SCREEN_HEIGHT * scale) as u32;

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("rust-nes-emulator", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    while let Some(e) = window.next() {
        // 描画
        if let Some(_) = e.render_args() {
            // 1frameの実行時間を控える
            let start = Instant::now();
            // エミュを進める
            let mut total_cycle: usize = 0;
            while total_cycle < cycle_for_draw_once {
                let cpu_cycle = usize::from(cpu.step(&mut cpu_sys, &ppu));
                ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut fb);
                total_cycle = total_cycle + cpu_cycle;
            }
            let duration = start.elapsed();

            // 画面更新(毎回やらんほうが良さげ?)
            for j in 0..VISIBLE_SCREEN_HEIGHT {
                for i in 0..VISIBLE_SCREEN_WIDTH {
                    let x = i as u32;
                    let y = j as u32;
                    let color = fb[j][i];
                    canvas.put_pixel(x, y, im::Rgba([color[0], color[1], color[2], 255]));
                }
            }
            // 書く
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([0.0; 4], g);
                image(&texture, c.transform.scale(scale as f64, scale as f64), g);
            });
            // windowとか
            window.set_title(format!("[rust-nes-emulator] pc:${:04X} fps:{:.*}", cpu.pc, 1, 1000.0 / (duration.as_millis() as f32)));
        }
        // ボタン入力
        if let Some(Button::Keyboard(key)) = e.press_args() {
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
                Key::E => { 
                    debugger_enable_fileout!("nes_gui.log".to_string());
                },
                Key::R => {
                    debugger_disable_fileout!();
                },
                _ => {},
            }
        };
    }
}