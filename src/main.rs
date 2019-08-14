extern crate rust_nes_emulator as emu;

use emu::io::memory::MemoryIo;

/* for desktop simulation driver */
// TODO: ある程度完成したらDesktopで動かす部分は別リポジトリに移動して、本リポジトリをライブラリとして参照する
struct VirtualMachine {
    rom: Vec<u8>
}
impl MemoryIo for VirtualMachine {
    fn read_u8(addr: u32) {}
    fn write_u8(addr: u32, data: u8) {}
}
/* for desktop simulation driver */

fn main() {
    println!("Hello, world!");
}
