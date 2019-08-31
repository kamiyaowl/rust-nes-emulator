#![allow(unused_macros)]

#[derive(Debug)]
pub enum PrintLevel {
    INFO,
    DEBUG,
}

#[derive(Debug)]
pub enum PrintFrom {
    CPU,
    PPU,
    APU,
    PAD,
    CASSETTE,
    SYSTEM,
    TEST,
    MAIN,
}

pub const DEBUGGER_OUT_PATH: &str ="emulator.log";

/// デバッグ出力ファイルのクリアとか
#[macro_export]
macro_rules! debugger_init {
    () => {
        if cfg!(not(no_std)) {
            use std::io::{BufWriter, Write};
            use std::fs;
            use fs::OpenOptions;
            // とりあえず消す
            fs::remove_file(DEBUGGER_OUT_PATH).unwrap_or_else(|why| {
                println!("[debugger_init] log delete error.");
                println!("{:?}", why.kind());
            });
            // そして作る
            let mut file = BufWriter::new(
                OpenOptions::new()
                .write(true)
                .create(true)
                .open(DEBUGGER_OUT_PATH)
                .unwrap()
            );
            file.write_all("rust-nes-emulator log\n".as_bytes()).unwrap();
            file.flush().unwrap();
        }
    };
}

/// debug_assertionsとfeatureの指定でbodyを実行するかどうか決める
/// `level` - PrintLevel
/// `from`  - PrintFrom
/// `body`  - 実際にStringを返却するコードブロック
#[macro_export]
macro_rules! debugger_print {
    ($level: expr, $from: expr, $body: expr) => {
        let is_print = cfg!(debug_assertions) && (match $level {
                PrintLevel::INFO => {
                    // 普通に出力する
                    true
                },
                PrintLevel::DEBUG => {
                    // 普段から出すとうるさいのでFromと組み合わせて返す
                    match $from {
                        PrintFrom::CPU      => cfg!(feature = "debug_cpu"),
                        PrintFrom::PPU      => cfg!(feature = "debug_ppu"),
                        PrintFrom::APU      => cfg!(feature = "debug_apu"),
                        PrintFrom::PAD      => cfg!(feature = "debug_pad"),
                        PrintFrom::CASSETTE => cfg!(feature = "debug_cassette"),
                        PrintFrom::SYSTEM   => cfg!(feature = "debug_system"),
                        PrintFrom::TEST     => cfg!(feature = "debug_test"),
                        PrintFrom::MAIN     => cfg!(feature = "debug_main"),
                    }
                },
            });
        // stdoutは出力フィルタを無事通過した場合だけ
        let print_str = format!("[{:?}][{:?}]{}\n", $level, $from, $body);
        if is_print && cfg!(not(no_std)) { // とりあえずnostdにしておく(semihostingとかは使えそうである)
            print!("{}", print_str);
        }    
        // ファイル出力、こちらはフィルタ無視
        if cfg!(not(no_std)) {
            use std::fs;
            use std::io::{BufWriter, Write};
            use fs::OpenOptions;
            let mut file = BufWriter::new(
                OpenOptions::new()
                .write(true)
                .append(true)
                .open(DEBUGGER_OUT_PATH)
                .unwrap()
            );
            file.write_all(print_str.as_bytes()).unwrap();
            file.flush().unwrap();
        }
    }
}