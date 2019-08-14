pub trait MemoryIo {
    fn read_u8(addr: u32);
    fn write_u8(addr: u32, data: u8);
}