extern crate rust_nes_emulator;

use rust_nes_emulator::*;
use rust_nes_emulator::interface::*;

// for read ines file
use std::fs::File;
use std::io::Read;

// for save screenshot
extern crate bmp;
use bmp::{Image, Pixel};

// for GUI
extern crate piston_window;
extern crate image as im;
extern crate nfd;

use piston_window::*;
use nfd::Response;
use std::time::Instant;

/// NESファイルを読み込んでカセットにロードさせます
fn load_cassette(cassette: &mut Cassette, path: String) {
    
    let mut file = File::open(path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    // casseteに展開
    if !cassette.from_ines_binary(|addr: usize| buf[addr]) {
        panic!("ines binary read error");
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
    let _ = img.save(path);
}

fn main() {
    // let rom_path = "roms/nes-test-roms/other/nestest.nes".to_string();
    // let rom_path = "roms/nes-test-roms/scrolltest/sssscroll.nes".to_string();
    let rom_path = "../roms/my_dump/mario.nes".to_string();
    // let rom_path = "roms/my_dump/donkey.nes".to_string();

    let rom_exists = std::path::Path::new(&rom_path).is_file();

    // emu
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    if rom_exists {
        load_cassette(&mut cpu_sys.cassette, rom_path);
    } else {
        // 選んで
        let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
            panic!(e);
        });
        match result {
            Response::Okay(file_path) => {
                load_cassette(&mut cpu_sys.cassette, file_path.clone());
            },
            _ => panic!("no input file"),
        }
    }
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
    // グリッド表示有無
    let mut is_show_grid = false;

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
            let emulate_duration = start.elapsed();

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
                // debug用にtile境界線とか入れる
                if is_show_grid {
                    for i in 0..SCREEN_TILE_WIDTH {
                        let x = (PIXEL_PER_TILE * i) as f64 * (scale as f64);
                        line([1.0, 1.0, 1.0, 1.0], 0.5, [x, 0.0, x, (scale * VISIBLE_SCREEN_HEIGHT) as f64], c.transform, g);
                    }
                    for j in 0..SCREEN_TILE_HEIGHT {
                        let y = (PIXEL_PER_TILE * j) as f64 * (scale as f64);
                        line([1.0, 1.0, 1.0, 1.0], 0.5, [0.0, y, (scale * VISIBLE_SCREEN_WIDTH) as f64, y], c.transform, g);
                    }
                }
            });
            // windowとか
            let emulate_fps = 1000.0 / (emulate_duration.as_millis() as f32);
            window.set_title(format!("[rust-nes-emulator] pc:${:04X} emu_fps:{:.*}", cpu.pc, 1, emulate_fps));
        }
        // ボタン入力
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::J => { cpu_sys.pad1.release_button(PadButton::A) },
                Key::K => { cpu_sys.pad1.release_button(PadButton::B) },
                Key::U => { cpu_sys.pad1.release_button(PadButton::Select) },
                Key::I => { cpu_sys.pad1.release_button(PadButton::Start) },
                Key::W => { cpu_sys.pad1.release_button(PadButton::Up) },
                Key::S => { cpu_sys.pad1.release_button(PadButton::Down) },
                Key::A => { cpu_sys.pad1.release_button(PadButton::Left) },
                Key::D => { cpu_sys.pad1.release_button(PadButton::Right) },
                _ => {},
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::J => { cpu_sys.pad1.push_button(PadButton::A) },
                Key::K => { cpu_sys.pad1.push_button(PadButton::B) },
                Key::U => { cpu_sys.pad1.push_button(PadButton::Select) },
                Key::I => { cpu_sys.pad1.push_button(PadButton::Start) },
                Key::W => { cpu_sys.pad1.push_button(PadButton::Up) },
                Key::S => { cpu_sys.pad1.push_button(PadButton::Down) },
                Key::A => { cpu_sys.pad1.push_button(PadButton::Left) },
                Key::D => { cpu_sys.pad1.push_button(PadButton::Right) },
                Key::P => { 
                    save_framebuffer(&fb, "run_gui_ss.bmp".to_string());
                 },
                Key::G => {
                    is_show_grid = !is_show_grid;
                },
                Key::R => {
                    // Reset
                    cpu.reset();
                    cpu_sys.reset();
                    ppu.reset();
                    video_sys.reset();
                    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);
                },
                Key::O => {
                    // 別のファイルを開いてリセットする
                    let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                        panic!(e);
                    });
                    match result {
                        Response::Okay(file_path) => {
                            load_cassette(&mut cpu_sys.cassette, file_path.clone());
                            cpu.reset();
                            cpu_sys.reset();
                            ppu.reset();
                            video_sys.reset();
                            cpu.interrupt(&mut cpu_sys, Interrupt::RESET);
                        },
                        _ => {},
                    }
                }
                _ => {},
            }
        };
    }
}