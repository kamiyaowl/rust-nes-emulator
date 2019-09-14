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

/// 外部から内容の変更やクリアが可能
pub trait EmulateControl {
    /// 内部変数を強制的にリセットします。リセットベクタに飛ぶ挙動ではありません。
    fn reset(&mut self);
    /// dumpに必要なサイズを返します
    /// 実装者は確保した領域から、dumpの割当をこの関数を使って指定します
    fn get_dump_size() -> usize;
    /// 内部状態をcallbackに出力します
    /// * `read_callback` - (address, data)が呼ばれる
    fn dump(&self, read_callback: impl Fn(usize, u8));
    /// 内部状態をcallbackから復元します
    /// * `write_callback - (address)が呼ばれるので対応するdataを返す
    fn restore(&mut self, write_callback: impl Fn(usize) -> u8);
}