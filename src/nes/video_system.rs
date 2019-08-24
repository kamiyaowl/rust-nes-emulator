use super::interface::*;
use super::cassette::*;

pub const PATTERN_TABLE_BASE_ADDR     : u16 = 0x0000;
pub const NAME_TABLE_BASE_ADDR        : u16 = 0x2000;
pub const NAME_TABLE_MIRROR_BASE_ADDR : u16 = 0x3000;
pub const PALETTE_TABLE_BASE_ADDR     : u16 = 0x3f00;
pub const VIDEO_ADDRESS_SIZE          : u16 = 0x4000;

pub const NAME_TABLE_SIZE     : usize = 0x0400;
pub const NUM_OF_NAME_TABLE   : usize = 2;
pub const PALETTE_SIZE        : usize = 0x0020;

pub struct VideoSystem {
    // 0x0000 - 0x1fff 
    // pattern table 0/1
    // cassetteのCHR-RAMを読む

    /// 0x2000-0x2fff
    /// name table 0/1/2/3 (0x400が4面)
    /// 実際には2面しか持っていないのでカセットのミラーリング設定を引用
    /// 0x3000-0x3effは0x2000からのミラー
    pub nametables: [[u8; NAME_TABLE_SIZE]; NUM_OF_NAME_TABLE],

    /// 0x3f00 - 0x3f1f
    /// Palette
    /// 0x3f00 - 0x3fff領域でミラー
    pub palette: [u8; PALETTE_SIZE],
}

impl Default for VideoSystem {
    fn default() -> Self {
        Self {
            nametables: [[0; NAME_TABLE_SIZE]; NUM_OF_NAME_TABLE],
            palette: [0; PALETTE_SIZE],
        }
    }
}

impl EmulateControl for VideoSystem {
    fn reset(&mut self) {
        self.nametables = [[0; NAME_TABLE_SIZE]; NUM_OF_NAME_TABLE];
        self.palette = [0; PALETTE_SIZE];
    }
    fn get_dump_size() -> usize {
        unimplemented!();
    }
    fn dump(&self, _read_callback: impl Fn(usize, u8)) {
        unimplemented!();
    }
    fn restore(&mut self, _write_callback: impl Fn(usize) -> u8) {
        // TODO: #14
        unimplemented!();
    }
}

impl VideoSystem {
    /// NameTable Mirrorのアドレス変換をします
    /// 戻り値: (table_index[0,1,2,3のどれか], offset[中身のindex])
    /// 
    /// [A, B]: A-0x2000, B-0x2400
    /// [C, D]: C-0x2800, D-0x2c00
    fn convert_name_table_addr(&self, mirror_mode: NameTableMirror, addr: u16) -> (usize, usize) {
        debug_assert!(addr >= NAME_TABLE_BASE_ADDR);
        debug_assert!(addr < NAME_TABLE_MIRROR_BASE_ADDR);

        // offsetはすぐわかるはず
        let offset = usize::from(addr) % NAME_TABLE_SIZE;
        // でかいとこはは頑張らないとわからんな
        let table_index = match mirror_mode {
            NameTableMirror::Horizontal => {
                // [A, A]
                // [B, B]
                if addr < 0x2800 {
                    0
                } else {
                    1
                }
            },
            NameTableMirror::Vertical => {
                // [A, B]
                // [A, B]
                let tmp_addr = if addr >= 0x2800 { addr - 0x800 } else { addr };// とりあえず上の領域で考える
                if tmp_addr < 0x2400 {
                    0
                } else {
                    1
                }
            },
            NameTableMirror::SingleScreen => {
                // [A, A]
                // [A, A]
                0
            },
            NameTableMirror::FourScreen => {
                // [A, B]
                // [C, D]
                usize::from((addr - 0x2000) / 4)
            },
            _ => {
                unimplemented!();
            }
        };
        (table_index, offset)
    }
    pub fn read_u8(&self, cassette: &mut Cassette, addr: u16) -> u8 {
        debug_assert!(addr < VIDEO_ADDRESS_SIZE);

        if addr < NAME_TABLE_BASE_ADDR {
            cassette.read_video_u8(addr)
        } else if addr < NAME_TABLE_MIRROR_BASE_ADDR {
            let (index, offset) = self.convert_name_table_addr(cassette.nametable_mirror, addr);
            self.nametables[index][offset]
        } else if addr < PALETTE_TABLE_BASE_ADDR {
            // 0x3000 -> 0x2000にミラーする
            let (index, offset) = self.convert_name_table_addr(cassette.nametable_mirror, addr - 0x1000);
            self.nametables[index][offset]
        } else {
            // Palette with mirroring
            let index = usize::from(addr - PALETTE_TABLE_BASE_ADDR) % PALETTE_SIZE;
            self.palette[index]
        }
    }
    pub fn write_u8(&mut self, cassette: &mut Cassette, addr: u16, data: u8) {
        debug_assert!(addr < VIDEO_ADDRESS_SIZE);

        if addr < NAME_TABLE_BASE_ADDR {
            cassette.write_video_u8(addr, data);
        } else if addr < NAME_TABLE_MIRROR_BASE_ADDR {
            let (index, offset) = self.convert_name_table_addr(cassette.nametable_mirror, addr);
            self.nametables[index][offset] = data;
        } else if addr < PALETTE_TABLE_BASE_ADDR {
            // 0x3000 -> 0x2000にミラーする
            let (index, offset) = self.convert_name_table_addr(cassette.nametable_mirror, addr - 0x1000);
            self.nametables[index][offset] = data;
        } else {
            // Palette with mirroring
            let index = usize::from(addr - PALETTE_TABLE_BASE_ADDR) % PALETTE_SIZE;
            self.palette[index] = data;
        }
    }
}