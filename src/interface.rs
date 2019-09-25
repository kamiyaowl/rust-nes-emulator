/// System Bus経由でR/Wする機能を提供します
/// 実装する際は、CPU命令のままであるoffset付きでアクセスすること
pub trait SystemBus {
    fn read_u8(&mut self, addr: u16, is_nondestructive: bool) -> u8;
    fn write_u8(&mut self, addr: u16, data: u8, is_nondestructive: bool);
}

/// Video Bus経由でR/Wする機能を提供します
/// addrは0x0000 ~ 0x3fffの範囲内
pub trait VideoBus {
    fn read_video_u8(&mut self, addr: u16) -> u8;
    fn write_video_u8(&mut self, addr: u16, data: u8);
}

pub trait EmulateControl {
    /// 内部変数を強制的にリセットします。リセットベクタに飛ぶ挙動ではありません。
    fn reset(&mut self);
}

#[cfg(feature = "unsafe-opt")]
#[allow(unused_macros)]
macro_rules! arr_read {
    ($arr:expr, $index:expr) => {
        unsafe { *$arr.get_unchecked($index) }
    };
}

#[cfg(feature = "unsafe-opt")]
#[allow(unused_macros)]
macro_rules! arr_write {
    ($arr:expr, $index:expr, $data:expr) => {
        unsafe { *$arr.get_unchecked_mut($index) = $data }
    };
}

#[cfg(not(feature = "unsafe-opt"))]
#[allow(unused_macros)]
macro_rules! arr_read {
    ($arr:expr, $index:expr) => {
        $arr[$index]
    };
}

#[cfg(not(feature = "unsafe-opt"))]
#[allow(unused_macros)]
macro_rules! arr_write {
    ($arr:expr, $index:expr, $data:expr) => {
        $arr[$index] = $data
    };
}
