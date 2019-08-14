/// System Bus経由でR/Wできる
pub trait SystemBus {
    fn read_u8(&self, addr: usize) -> u8;
    fn write_u8(&mut self, addr: usize, data: u8);
}

/// 外部から内容の変更やクリアが可能
pub trait EmulateControl {
    /// 内部状態をリセットします
    fn reset(&mut self);
    /// 内部状態をcallbackに出力します
    /// * `read_callback` - (address, data)が呼ばれる
    fn store(&self, read_callback: fn(usize, u8));
    /// 内部状態をcallbackから復元します
    /// * `write_callback - (address)が呼ばれるので対応するdataを返す
    fn restore(&mut self, write_callback: fn(usize) -> u8);
}