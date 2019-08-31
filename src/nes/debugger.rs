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

/// debug_assertionsとfeatureの指定でbodyを実行するかどうか決める
/// `level` - PrintLevel
/// `from`  - PrintFrom
/// `body`  - 実際に出力するコードブロック
#[macro_export]
macro_rules! config_filter {
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
                        PrintFrom::PAD      => cfg!(feature = "debug_pad"),
                        PrintFrom::CASSETTE => cfg!(feature = "debug_cassette"),
                        PrintFrom::SYSTEM   => cfg!(feature = "debug_system"),
                        PrintFrom::TEST     => cfg!(feature = "debug_test"),
                        PrintFrom::MAIN     => cfg!(feature = "debug_main"),
                    }
                },
                _ => unimplemented!(),
            });
        // 出力するか
        if is_print {
            print!("[{:?}][{:?}] ", $level, $from);
            $body;
        }    
    }
}