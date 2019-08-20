use super::system::*;

/// PPU Register Implement
/// 0x2000 - 0x2007
/// PPU本体の実装向けです。CPUから本レジスタを本関数を通して読むことはありません(STA, STX, STYなどで読むのが普通)
impl System {
    /*************************** 0x2000: PPUCTRL ***************************/
    /// VBLANK発生時にNMI割り込みを出す
    /// oneshotではなく0x2002のVLANKフラグがある限り
    pub fn read_ppu_nmi_enable(&self) -> bool {
        (self.ppu_reg[0] & 0x80u8) != 0x80u8
    }
    pub fn read_ppu_is_master(&self) -> bool {
        (self.ppu_reg[0] & 0x40u8) != 0x40u8
    }
    pub fn read_ppu_sprite_height(&self) -> u8 {
        if (self.ppu_reg[0] & 0x20u8) != 0x20u8 { 16 } else { 8 }
    }
    pub fn read_ppu_bg_pattern_table_addr(&self) -> u16 {
        if (self.ppu_reg[0] & 0x10u8) != 0x10u8 { 0x1000u16 } else { 0x0000u16 }
    }
    pub fn read_ppu_sprite_pattern_table_addr(&self) -> u16 {
        if (self.ppu_reg[0] & 0x08u8) != 0x08u8 { 0x1000u16 } else { 0x0000u16 }
    }
    /// PPUのアドレスインクリメント数 0:+1, horizontal, 1:+32 vertical
    pub fn read_ppu_addr_increment(&self) -> u8 {
        if (self.ppu_reg[0] & 0x04u8) != 0x04u8 { 32u8 } else { 1u8 }
    }
    pub fn read_ppu_name_table_addr(&self) -> u16 {
        match self.ppu_reg[0] & 0x03u8 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("invalid name table addr index"),
        }
    }
}

