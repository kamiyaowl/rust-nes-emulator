extern crate rust_nes_emulator;
use rust_nes_emulator::interface::*;
use rust_nes_emulator::*;

// for read ines file
use std::fs::File;
use std::io::Read;

// for save screenshot
extern crate bmp;
use bmp::{Image, Pixel};

/// NESファイルを読み込んでカセットにロードさせます
#[allow(dead_code)]
fn load_cassette(cassette: &mut Cassette, path: String) {
    let mut file = File::open(path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    // casseteに展開
    if !cassette.from_ines_binary(|addr: usize| buf[addr]) {
        panic!("ines binary read error");
    }
}

/// FrameBufferの中身をコンソール出力します。色があれば#, なければ.が出力されます
#[allow(dead_code)]
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
#[allow(dead_code)]
fn save_framebuffer(
    fb: &[[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    path: String,
) {
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

/// FrameBufferの中身を保存されたbmpファイルと比較します
#[allow(dead_code)]
fn validate_framebuffer(
    fb: &[[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    path: String,
) {
    let img = bmp::open(path).unwrap();

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
}

/// cpuだけで指定したサイクル流す
#[allow(dead_code)]
fn run_cpu_only(rom_path: String, cpu_steps: usize, validate: impl Fn(&Cpu, &System)) {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    load_cassette(&mut cpu_sys.cassette, rom_path);

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut cpu_cycle: usize = 0;
    for _i in 0..cpu_steps {
        cpu_cycle = cpu_cycle + usize::from(cpu.step(&mut cpu_sys, &ppu));
    }
    validate(&cpu, &cpu_sys);
}

/// 指定したフレーム数だけ流す
#[allow(dead_code)]
fn run_cpu_ppu(
    rom_path: String,
    save_path: String,
    frame_count: usize,
    validate: impl Fn(&Cpu, &System, &[[[u8; 3]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]),
) {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    load_cassette(&mut cpu_sys.cassette, rom_path);

    cpu.reset();
    cpu_sys.reset();
    ppu.reset();
    video_sys.reset();
    cpu.interrupt(&mut cpu_sys, Interrupt::RESET);

    let mut fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];

    // cpuを基準にppuを動かしてあげる
    let cycle_for_draw_once = CPU_CYCLE_PER_LINE * usize::from(RENDER_SCREEN_HEIGHT + 1);
    for _i in 0..frame_count {
        let mut total_cycle: usize = 0;
        while total_cycle < cycle_for_draw_once {
            let cpu_cycle = usize::from(cpu.step(&mut cpu_sys, &ppu));
            ppu.step(cpu_cycle, &mut cpu, &mut cpu_sys, &mut video_sys, &mut fb);

            total_cycle = total_cycle + cpu_cycle;
        }
    }

    print_framebuffer(&fb);
    save_framebuffer(&fb, save_path);

    validate(&cpu, &cpu_sys, &fb);
}

/// nestestを起動して、テストを実行してスクショ比較する
#[allow(dead_code)]
fn run_nestest(rom_path: String) {
    let mut cpu: Cpu = Default::default();
    let mut cpu_sys: System = Default::default();
    let mut ppu: Ppu = Default::default();
    let mut video_sys: VideoSystem = Default::default();

    load_cassette(&mut cpu_sys.cassette, rom_path);

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

            total_cycle = total_cycle + cpu_cycle;
        }
        match i {
            4 => {
                // 起動画像が出るはず
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_normal_menu.bmp".to_string());
                let _ =
                    validate_framebuffer(&fb, "../screenshot/nestest_normal_menu.bmp".to_string());
            }
            7 => {
                // テスト開始ボタン押させる
                cpu_sys.pad1.push_button(PadButton::Start);
            }
            25 => {
                cpu_sys.pad1.release_button(PadButton::Start);
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_normal.bmp".to_string());
                let _ = validate_framebuffer(&fb, "../screenshot/nestest_normal.bmp".to_string());
            }
            26 => {
                // unofficial testに遷移させる
                cpu_sys.pad1.push_button(PadButton::Select);
            }
            30 => {
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_extra_menu.bmp".to_string());
                let _ =
                    validate_framebuffer(&fb, "../screenshot/nestest_extra_menu.bmp".to_string());
            }
            35 => {
                // テスト開始ボタン押させる
                cpu_sys.pad1.push_button(PadButton::Start);
            }
            55 => {
                cpu_sys.pad1.release_button(PadButton::Start);
                print_framebuffer(&fb);
                save_framebuffer(&fb, "nestest_extra.bmp".to_string());
                let _ = validate_framebuffer(&fb, "../screenshot/nestest_extra.bmp".to_string());
            }
            _ => {}
        };
    }
}

/// hello worldのromで、一通りの処理が終わって無限ループまでたどり着くことを確認する
#[test]
fn test_run_hello_cpu() {
    run_cpu_only("../roms/other/hello.nes".to_string(), 175, |cpu, _sys| {
        // 170step以降はJMPで無限ループしているはず
        assert_eq!(0x804e, cpu.pc);
        assert_eq!(0x01ff, cpu.sp);
        assert_eq!(0x1e, cpu.a);
        assert_eq!(0x0d, cpu.x);
        assert_eq!(0x00, cpu.y);
        assert_eq!(0x34, cpu.p);
    })
}

/// 画面上にhello world!が正しく表示されることを確認する
#[test]
fn test_run_hello_ppu() {
    run_cpu_ppu(
        "../roms/other/hello.nes".to_string(),
        "test_run_hello_ppu.bmp".to_string(),
        1,
        |cpu, _sys, fb| {
            // 170step以降はJMPで無限ループしているはず
            assert_eq!(0x804e, cpu.pc);
            assert_eq!(0x01ff, cpu.sp);
            assert_eq!(0x1e, cpu.a);
            assert_eq!(0x0d, cpu.x);
            assert_eq!(0x00, cpu.y);
            assert_eq!(0x34, cpu.p);
            // FBの結果を精査する
            let _ = validate_framebuffer(fb, "../screenshot/hello.bmp".to_string());
        },
    )
}

/// nestestがすべてPassできることを確認する
#[test]
fn test_run_nestest() {
    run_nestest("../roms/nes-test-roms/other/nestest.nes".to_string())
}

/// マリオのタイトル画面が正しく表示されること
#[test]
#[ignore]
fn test_run_mario_title() {
    run_cpu_ppu(
        "../roms/my_dump/mario.nes".to_string(),
        "mario_title.bmp".to_string(),
        100,
        |_cpu, _sys, fb| {
            // FBの結果を精査する
            let _ = validate_framebuffer(fb, "../screenshot/mario.bmp".to_string());
        },
    )
}

/// マリオのデモ映像が正しく表示されること
#[test]
#[ignore]
fn test_run_mario_demo() {
    run_cpu_ppu(
        "../roms/my_dump/mario.nes".to_string(),
        "mario_demo.bmp".to_string(),
        1000,
        |_cpu, _sys, fb| {
            // FBの結果を精査する
            let _ = validate_framebuffer(fb, "../screenshot/mario_demo.bmp".to_string());
        },
    )
}
