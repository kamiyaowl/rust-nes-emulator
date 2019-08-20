use super::interface::{SystemBus, EmulateControl};

#[derive(Copy, Clone)]
pub struct Cassette {
    pub mapper: Mapper,
    // TODO: enum Mapperに持たせたほうが...
    pub prg_rom: [u8; 0x8000], // 32KB
    pub chr_rom: [u8; 0x2000], // 8K
}

/// Cassete and mapper implement
/// https://wiki.nesdev.com/w/index.php/List_of_mappers
#[derive(Copy, Clone)]
pub enum Mapper {
    Unknown,
    /// Mapper0: no mapper
    Nrom,
}

impl Cassette {
    /// inesファイルから読み出してメモリ上に展開します
    /// 組み込み環境でRAM展開されていなくても利用できるように、多少パフォーマンスを犠牲にしてもclosure経由で読み出します
    pub fn from_ines_binary(&mut self, read_closure: impl Fn(usize) -> u8) -> bool {
        // header : 16byte
        // trainer: 0 or 512byte
        // prg rom: prg_rom_size * 16KB(0x4000)
        // chr rom: prg_rom_size * 8KB(0x2000)
        // playchoise inst-rom: 0 or 8192byte(8KB)
        // playchoise prom: 16byte

        // header check
        if read_closure(0) != 0x4e { // N
            return false;
        }
        if read_closure(1) != 0x45 { // E
            return false;
        }
        if read_closure(2) != 0x53 { // S
            return false;
        }
        if read_closure(3) != 0x1a { // character break
            return false;
        }
        let prg_rom_size = usize::from(read_closure(4)); // * 16KBしてあげる
        let chr_rom_size = usize::from(read_closure(5)); // * 8KBしてあげる
        let flags6       = read_closure(6);
        let _flags7      = read_closure(7);
        let _flags8      = read_closure(8);
        let _flags9      = read_closure(9);
        let _flags10     = read_closure(10);
        // 11~15 unused_padding
        debug_assert!(prg_rom_size > 0);

        // flags parsing
        let _is_mirroring_vertical        = (flags6 & 0x01) == 0x01;
        let _is_exists_battery_backed_ram = (flags6 & 0x02) == 0x02;
        let is_exists_trainer             = (flags6 & 0x04) == 0x04; // 512byte trainer at 0x7000-0x71ff

        // 領域計算
        let header_bytes  = 16;
        let trainer_bytes = if is_exists_trainer { 512 } else { 0 };
        let prg_rom_bytes = prg_rom_size    * 0x4000;
        let chr_rom_bytes = chr_rom_size    * 0x2000;
        let prg_rom_baseaddr = header_bytes + trainer_bytes;
        let chr_rom_baseaddr = header_bytes + trainer_bytes + prg_rom_bytes;

        // 現在はMapper0しか対応しない
        self.mapper = Mapper::Nrom;
        debug_assert!(prg_rom_bytes <= 0x8000);
        debug_assert!(chr_rom_bytes <= 0x2000);

        // Seq Readしか許さない場合、trainer領域を読み飛ばす
        if cfg!(debug_assertions) && cfg!(not(no_std)) {
            println!("[cassette][from ines bin] header_bytes:{:04x}", header_bytes);
            println!("[cassette][from ines bin] trainer_bytes:{:04x}", trainer_bytes);
            println!("[cassette][from ines bin] prg_rom_bytes:{:04x}", prg_rom_bytes);
            println!("[cassette][from ines bin] chr_rom_bytes:{:04x}", chr_rom_bytes);
            println!("[cassette][from ines bin] prg_rom_baseaddr:{:04x}", prg_rom_baseaddr);
            println!("[cassette][from ines bin] chr_rom_baseaddr:{:04x}", chr_rom_baseaddr);
        }
        // CPUで地道にコピーする
        for index in 0..prg_rom_bytes {
            let ines_binary_addr = prg_rom_baseaddr + index;
            self.prg_rom[index] = read_closure(ines_binary_addr);
            // NROM 16KBの場合、後半0xc0000~0xffffはミラーしてあげないとだめ
            if prg_rom_size == 1 {
                self.prg_rom[index + 0x4000] = read_closure(ines_binary_addr);
            }
        }
        for index in 0..chr_rom_bytes {
            let ines_binary_addr = chr_rom_baseaddr + index;
            self.chr_rom[index] = read_closure(ines_binary_addr);
        }

        // やったね
        true
    }
}

impl SystemBus for Cassette {
    fn read_u8(&self, addr: u16) -> u8 {
        debug_assert!(addr >= 0x8000);
        let index = usize::from(addr - 0x8000);
        match self.mapper {
            Mapper::Nrom => self.prg_rom[index],
            _ => unimplemented!(),
        }
    }
    fn write_u8(&mut self, addr: u16, data: u8) {
        debug_assert!(addr >= 0x8000);
        let index = usize::from(addr - 0x8000);
        match self.mapper {
            Mapper::Nrom => self.prg_rom[index] = data,
            _ => unimplemented!(),
        }
    }
}

impl EmulateControl for Cassette {
    fn reset(&mut self){
        self.prg_rom = [0; 0x8000];
        self.chr_rom = [0; 0x2000];
    }
    fn get_dump_size() -> usize {
        unimplemented!();
    }
    fn dump(&self, _read_callback: impl Fn(usize, u8)) {
        unimplemented!();
    }
    fn restore(&mut self, _write_callback: impl Fn(usize) -> u8) {
        unimplemented!();
    }
}