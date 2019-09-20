use super::interface::*;

pub const PRG_ROM_MAX_SIZE: usize = 0x8000;
pub const CHR_ROM_MAX_SIZE: usize = 0x2000;
pub const BATTERY_PACKED_RAM_MAX_SIZE: usize = 0x2000;

pub const PRG_ROM_SYSTEM_BASE_ADDR: u16 = 0x8000;
pub const BATTERY_PACKED_RAM_BASE_ADDR: u16 = 0x6000;

pub const INES_TRAINER_DATA_SIZE: usize = 0x0200;

#[derive(Copy, Clone)]
pub enum Mapper {
    Unknown,
    /// Mapper0: no mapper
    Nrom,
}

#[derive(Copy, Clone)]
pub enum NameTableMirror {
    Unknown,
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen,
}
/// Cassete and mapper implement
/// https://wiki.nesdev.com/w/index.php/List_of_mappers
#[derive(Clone)]
pub struct Cassette {
    // Mapperの種類
    pub mapper: Mapper,
    /// Video領域での0x2000 ~ 0x2effのミラーリング設定
    pub nametable_mirror: NameTableMirror,
    /// 0x6000 ~ 0x7fffのカセット内RAMを有効化する
    pub is_exists_battery_backed_ram: bool,

    // data size
    pub prg_rom_bytes: usize,
    pub chr_rom_bytes: usize,
    // datas
    pub prg_rom: [u8; PRG_ROM_MAX_SIZE], // 32KB
    pub chr_rom: [u8; CHR_ROM_MAX_SIZE], // 8K
    pub battery_packed_ram: [u8; BATTERY_PACKED_RAM_MAX_SIZE],
}

impl Default for Cassette {
    fn default() -> Self {
        Self {
            mapper: Mapper::Unknown,
            nametable_mirror: NameTableMirror::Unknown,
            is_exists_battery_backed_ram: false,

            prg_rom_bytes: 0,
            chr_rom_bytes: 0,

            prg_rom: [0; PRG_ROM_MAX_SIZE],
            chr_rom: [0; CHR_ROM_MAX_SIZE],
            battery_packed_ram: [0; BATTERY_PACKED_RAM_MAX_SIZE],
        }
    }
}

impl Cassette {
    /// inesファイルから読み出してメモリ上に展開します
    /// 組み込み環境でRAM展開されていなくても利用できるように、多少パフォーマンスを犠牲にしてもclosure経由で読み出します
    pub fn from_ines_binary(&mut self, read_func: impl Fn(usize) -> u8) -> bool {
        // header : 16byte
        // trainer: 0 or 512byte
        // prg rom: prg_rom_size * 16KB(0x4000)
        // chr rom: prg_rom_size * 8KB(0x2000)
        // playchoise inst-rom: 0 or 8192byte(8KB)
        // playchoise prom: 16byte

        // header check
        if read_func(0) != 0x4e {
            // N
            return false;
        }
        if read_func(1) != 0x45 {
            // E
            return false;
        }
        if read_func(2) != 0x53 {
            // S
            return false;
        }
        if read_func(3) != 0x1a {
            // character break
            return false;
        }
        let prg_rom_size = usize::from(read_func(4)); // * 16KBしてあげる
        let chr_rom_size = usize::from(read_func(5)); // * 8KBしてあげる
        let flags6 = read_func(6);
        let _flags7 = read_func(7);
        let _flags8 = read_func(8);
        let _flags9 = read_func(9);
        let _flags10 = read_func(10);
        // 11~15 unused_padding
        debug_assert!(prg_rom_size > 0);

        // flags parsing
        let is_mirroring_vertical = (flags6 & 0x01) == 0x01;
        if is_mirroring_vertical {
            self.nametable_mirror = NameTableMirror::Vertical;
        } else {
            self.nametable_mirror = NameTableMirror::Horizontal;
        }
        self.is_exists_battery_backed_ram = (flags6 & 0x02) == 0x02; // 0x6000 - 0x7fffのRAMを使わせる
        let is_exists_trainer = (flags6 & 0x04) == 0x04; // 512byte trainer at 0x7000-0x71ff in ines file

        // 領域計算
        let header_bytes = 16;
        let trainer_bytes = if is_exists_trainer { 512 } else { 0 };
        let prg_rom_bytes = prg_rom_size * 0x4000; // 単位変換する
        let chr_rom_bytes = chr_rom_size * 0x2000; // 単位変換する
        let trainer_baseaddr = header_bytes;
        let prg_rom_baseaddr = header_bytes + trainer_bytes;
        let chr_rom_baseaddr = header_bytes + trainer_bytes + prg_rom_bytes;

        // 現在はMapper0しか対応しない
        self.mapper = Mapper::Nrom;
        debug_assert!(prg_rom_bytes <= PRG_ROM_MAX_SIZE);
        debug_assert!(chr_rom_bytes <= CHR_ROM_MAX_SIZE);

        // Battery Packed RAMの初期値
        if is_exists_trainer {
            // 0x7000 - 0x71ffに展開する
            for index in 0..INES_TRAINER_DATA_SIZE {
                let ines_binary_addr = trainer_baseaddr + index;
                self.prg_rom[index] = read_func(ines_binary_addr);
            }
        }

        // PRG-ROM
        for index in 0..prg_rom_bytes {
            let ines_binary_addr = prg_rom_baseaddr + index;
            self.prg_rom[index] = read_func(ines_binary_addr);
        }
        // CHR-ROM
        for index in 0..chr_rom_bytes {
            let ines_binary_addr = chr_rom_baseaddr + index;
            self.chr_rom[index] = read_func(ines_binary_addr);
        }

        // rom sizeをセットしとく
        self.prg_rom_bytes = prg_rom_bytes;
        self.chr_rom_bytes = chr_rom_bytes;

        // やったね
        true
    }
}

impl SystemBus for Cassette {
    fn read_u8(&mut self, addr: u16, _is_nondestructive: bool) -> u8 {
        if addr < PRG_ROM_SYSTEM_BASE_ADDR {
            debug_assert!(addr >= BATTERY_PACKED_RAM_BASE_ADDR);

            let index = usize::from(addr - BATTERY_PACKED_RAM_BASE_ADDR);
            self.battery_packed_ram[index]
        } else {
            debug_assert!(addr >= PRG_ROM_SYSTEM_BASE_ADDR);

            let index = usize::from(addr - PRG_ROM_SYSTEM_BASE_ADDR);
            // ROMが16KB場合のミラーリング
            if index < self.prg_rom_bytes {
                self.prg_rom[index]
            } else {
                self.prg_rom[index - self.prg_rom_bytes]
            }
        }
    }
    fn write_u8(&mut self, addr: u16, data: u8, _is_nondestructive: bool) {
        if addr < PRG_ROM_SYSTEM_BASE_ADDR {
            debug_assert!(addr >= BATTERY_PACKED_RAM_BASE_ADDR);

            let index = usize::from(addr - BATTERY_PACKED_RAM_BASE_ADDR);
            self.battery_packed_ram[index] = data
        } else {
            debug_assert!(addr >= PRG_ROM_SYSTEM_BASE_ADDR);

            let index = usize::from(addr - PRG_ROM_SYSTEM_BASE_ADDR);
            // ROMが16KB場合のミラーリング
            if index < self.prg_rom_bytes {
                self.prg_rom[index] = data;
            } else {
                self.prg_rom[index - self.prg_rom_bytes] = data;
            }
        }
    }
}
impl VideoBus for Cassette {
    fn read_video_u8(&mut self, addr: u16) -> u8 {
        let index = usize::from(addr);
        debug_assert!(index < CHR_ROM_MAX_SIZE);
        self.chr_rom[index]
    }
    /// CHR_RAM対応も込めて書き換え可能にしておく
    fn write_video_u8(&mut self, addr: u16, data: u8) {
        let index = usize::from(addr);
        debug_assert!(index < CHR_ROM_MAX_SIZE);
        self.chr_rom[index] = data;
    }
}

impl EmulateControl for Cassette {
    fn reset(&mut self) {
        self.mapper = Mapper::Unknown;
        self.nametable_mirror = NameTableMirror::Unknown;
        self.is_exists_battery_backed_ram = false;
        self.prg_rom_bytes = 0;
        self.chr_rom_bytes = 0;
        self.prg_rom = [0; PRG_ROM_MAX_SIZE];
        self.chr_rom = [0; CHR_ROM_MAX_SIZE];
        self.battery_packed_ram = [0; BATTERY_PACKED_RAM_MAX_SIZE];
    }
}
