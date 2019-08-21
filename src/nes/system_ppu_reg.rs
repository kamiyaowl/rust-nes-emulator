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
    /*************************** 0x2001: PPUMASK ***************************/
    /// sprite描画有効判定
    pub fn read_ppu_is_write_sprite(&self) -> bool {
        (self.ppu_reg[1] & 0x10u8) != 0x10u8
    }
    /// bg描画有効判定
    pub fn read_ppu_is_write_bg(&self) -> bool {
        (self.ppu_reg[1] & 0x08u8) != 0x08u8
    }
    /// 左端8pxでスプライトクリッピング
    pub fn read_ppu_is_clip_sprite_at_leftend(&self) -> bool {
        (self.ppu_reg[1] & 0x04u8) != 0x04u8
    }
    /// 左端8pxでbgクリッピング
    pub fn read_ppu_is_clip_bg_at_leftend(&self) -> bool {
        (self.ppu_reg[1] & 0x02u8) != 0x02u8
    }
    pub fn read_is_monochrome(&self) -> bool {
        (self.ppu_reg[1] & 0x01u8) != 0x01u8
    }
    /*************************** 0x2002: PPU_STATUS ***************************/
    /// VBlankフラグをみて、NMI割り込みしようね
    pub fn read_ppu_is_vblank(&self) -> bool {
        (self.ppu_reg[2] & 0x80u8) != 0x80u8
    }
    /// VBlankフラグを立てる、NMI割り込みもしようね
    pub fn write_ppu_is_vblank(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[2] = self.ppu_reg[2] | 0x80u8;
        } else {
            self.ppu_reg[2] = self.ppu_reg[2] & (!0x80u8);
        }
    }
    pub fn read_ppu_is_hit_sprite0(&self) -> bool {
        (self.ppu_reg[2] & 0x40u8) != 0x40u8
    }
    pub fn write_ppu_is_hit_sprite0(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[2] = self.ppu_reg[2] | 0x40u8;
        } else {
            self.ppu_reg[2] = self.ppu_reg[2] & (!0x40u8);
        }
    }
    /// line 261到達時のリセット用
    pub fn clear_ppu_status(&mut self) {
        self.ppu_reg[2] = 0x00u8;
    }
    /*************************** 0x2003: OAMADDR ***************************/
    pub fn read_ppu_oam_addr(&self) -> u8 {
        self.ppu_reg[3]
    }
    /*************************** 0x2004: OAMDATA ***************************/
    pub fn read_ppu_oam_data(&self) -> u8 {
        self.ppu_reg[4]
    }
    /*************************** 0x2005: PPUSCROLL ***************************/
    pub fn read_ppu_scroll(&self) -> (u8, u8) {
        (self.ppu_reg[5], self.ppu_scroll_y_reg)
    }
    /*************************** 0x2006: PPUADDR ***************************/
    pub fn read_ppu_addr(&self) -> u16 {
        (u16::from(self.ppu_reg[6]) << 8) | u16::from(self.ppu_addr_lower_reg)
    }
    /*************************** 0x2007: PPUDATA ***************************/
    pub fn read_ppu_data(&self) -> u8 {
        self.ppu_reg[7]
    }
    /*************************** 0x4014: OAM_DMA ***************************/
    /// DMA開始が必要かどうかと、転送元アドレスを返す
    /// 面倒なので読み取ったらtriggerは揮発させる
    pub fn read_oam_dma_trigger(&mut self) -> (bool, u16) {
        let start_addr = u16::from(self.io_reg[0x14]) << 8;
        if self.is_trigger_oam_dam {
            self.is_trigger_oam_dam = false;

            (true, start_addr)
        } else {
            (false, start_addr)
        }
    }


}

