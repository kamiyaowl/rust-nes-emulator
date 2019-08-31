use std::sync::RwLock;

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

lazy_static! {
    pub static ref debugger_out_path       : RwLock<String> = RwLock::new("emulator.log".to_string());
    pub static ref debugger_fileout_enable : RwLock<bool>   = RwLock::new(false);
}

/// デバッグ情報のファイル出力を有効化します
/// cargo testはマルチスレッドのため、同時に複数のテストでloggingをオンにすると色々バグるので使わないこと
#[macro_export]
macro_rules! debugger_enable_fileout {
    ($filepath: expr) => {
        if cfg!(not(no_std)) {
            use std::io::{BufWriter, Write};
            use std::fs;
            use fs::OpenOptions;
            // とりあえず消す
            fs::remove_file($filepath).unwrap_or_else(|why| {
                println!("[debugger_init] log delete error.");
                println!("{:?}", why.kind());
            });
            // そして作る
            let mut file = BufWriter::new(
                OpenOptions::new()
                .write(true)
                .create(true)
                .open($filepath)
                .unwrap()
            );
            file.write_all($filepath.as_bytes()).unwrap();
            file.write_all(": rust-nes-emulator log\n".as_bytes()).unwrap();
            file.flush().unwrap();
            // global変数に泣く泣くセット
            let mut debugger_out_path_ptr       = debugger_out_path.write().unwrap();
            let mut debugger_fileout_enable_ptr = debugger_fileout_enable.write().unwrap();
            *debugger_out_path_ptr = $filepath;
            *debugger_fileout_enable_ptr = true;
        }
    };
}

#[macro_export]
macro_rules! debugger_disable_fileout {
    () => {
        if cfg!(not(no_std)) {
            let mut debugger_out_path_ptr       = debugger_out_path.write().unwrap();
            let mut debugger_fileout_enable_ptr = debugger_fileout_enable.write().unwrap();
            *debugger_out_path_ptr = "emulator.log".to_string();
            *debugger_fileout_enable_ptr = false;
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
            // debug print有効化されてたらやる
            let debugger_out_path_ptr       = debugger_out_path.read().unwrap();
            let debugger_fileout_enable_ptr = debugger_fileout_enable.read().unwrap();
            if *debugger_fileout_enable_ptr {
                let mut file = BufWriter::new(
                    OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open((*debugger_out_path_ptr).clone())
                    .unwrap()
                );
                file.write_all(print_str.as_bytes()).unwrap();
                file.flush().unwrap();                    
            }
        }
    }
}