use super::system::*;

pub const PPU_CTRL_OFFSET: usize = 0x00;
pub const PPU_MASK_OFFSET: usize = 0x01;
pub const PPU_STATUS_OFFSET: usize = 0x02;
pub const PPU_OAMADDR_OFFSET: usize = 0x03;
pub const PPU_OAMDATA_OFFSET: usize = 0x04;
pub const PPU_SCROLL_OFFSET: usize = 0x05;
pub const PPU_ADDR_OFFSET: usize = 0x06;
pub const PPU_DATA_OFFSET: usize = 0x07;
pub const APU_IO_OAM_DMA_OFFSET: usize = 0x14;

/// PPU Register Implement
/// 0x2000 - 0x2007
/// PPU本体の実装向けです。CPUから本レジスタを本関数を通して読むことはありません(STA, STX, STYなどで読むのが普通)
impl System {
    /*************************** 0x2000: PPUCTRL ***************************/
    /// VBLANK発生時にNMI割り込みを出す
    /// oneshotではなく0x2002のVLANKフラグがある限り
    pub fn read_ppu_nmi_enable(&self) -> bool {
        (self.ppu_reg[PPU_CTRL_OFFSET] & 0x80u8) == 0x80u8
    }
    /// 多分エミュだと使わない
    pub fn read_ppu_is_master(&self) -> bool {
        (self.ppu_reg[PPU_CTRL_OFFSET] & 0x40u8) == 0x40u8
    }
    /// 8もしくは16
    pub fn read_ppu_sprite_height(&self) -> u8 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x20u8) == 0x20u8 {
            16
        } else {
            8
        }
    }
    pub fn read_ppu_bg_pattern_table_addr(&self) -> u16 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x10u8) == 0x10u8 {
            0x1000u16
        } else {
            0x0000u16
        }
    }
    pub fn read_ppu_sprite_pattern_table_addr(&self) -> u16 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x08u8) == 0x08u8 {
            0x1000u16
        } else {
            0x0000u16
        }
    }
    /// PPUのアドレスインクリメント数 0:+1, horizontal, 1:+32 vertical
    pub fn read_ppu_addr_increment(&self) -> u8 {
        if (self.ppu_reg[PPU_CTRL_OFFSET] & 0x04u8) == 0x04u8 {
            32u8
        } else {
            1u8
        }
    }
    pub fn read_ppu_name_table_base_addr(&self) -> u16 {
        match self.ppu_reg[PPU_CTRL_OFFSET] & 0x03u8 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("invalid name table addr index"),
        }
    }
    /*************************** 0x2001: PPUMASK ***************************/
    // 論理が逆っぽいね。0がhide

    /// sprite描画有効判定
    pub fn read_ppu_is_write_sprite(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x10u8) == 0x10u8
    }
    /// bg描画有効判定
    pub fn read_ppu_is_write_bg(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x08u8) == 0x08u8
    }
    /// 左端8pxでスプライトクリッピング
    pub fn read_ppu_is_clip_sprite_leftend(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x04u8) != 0x04u8
    }
    /// 左端8pxでbgクリッピング
    pub fn read_ppu_is_clip_bg_leftend(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x02u8) != 0x02u8
    }
    pub fn read_is_monochrome(&self) -> bool {
        (self.ppu_reg[PPU_MASK_OFFSET] & 0x01u8) == 0x01u8
    }
    /*************************** 0x2002: PPU_STATUS ***************************/
    /// VBlankフラグをみて、NMI割り込みしようね
    /// CPUからPPU_STATUSを読みだした際の自動クリアなので、この関数を呼んでもクリアされない
    pub fn read_ppu_is_vblank(&self) -> bool {
        (self.ppu_reg[PPU_STATUS_OFFSET] & 0x80u8) == 0x80u8
    }
    /// VBlankフラグを立てる、NMI割り込みもしようね
    pub fn write_ppu_is_vblank(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x80u8;
        } else {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x80u8);
        }
    }
    /// Sprite0描画中かどうか
    pub fn read_ppu_is_hit_sprite0(&self) -> bool {
        (self.ppu_reg[PPU_STATUS_OFFSET] & 0x40u8) == 0x40u8
    }
    pub fn write_ppu_is_hit_sprite0(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x40u8;
        } else {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x40u8);
        }
    }
    /// scanline上のSprite数が8個より大きいか
    pub fn read_ppu_is_sprite_overflow(&self) -> bool {
        (self.ppu_reg[PPU_STATUS_OFFSET] & 0x20u8) == 0x20u8
    }
    pub fn write_ppu_is_sprite_overflow(&mut self, is_set: bool) {
        if is_set {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] | 0x20u8;
        } else {
            self.ppu_reg[PPU_STATUS_OFFSET] = self.ppu_reg[PPU_STATUS_OFFSET] & (!0x20u8);
        }
    }
    /// line 261到達時のリセット用
    pub fn clear_ppu_status(&mut self) {
        self.ppu_reg[PPU_STATUS_OFFSET] = 0x00u8;
    }
    /*************************** 0x2003: OAMADDR ***************************/
    pub fn read_ppu_oam_addr(&self) -> u8 {
        self.ppu_reg[PPU_OAMADDR_OFFSET]
    }
    /*************************** 0x2004: OAMDATA ***************************/
    /// OAM_DATAの書き換えがあったかを示すフラグもついてくるよ(自動で揮発します)
    /// is_read, is_write, data
    pub fn read_oam_data(&mut self) -> (bool, bool, u8) {
        // Write優先でフラグ管理して返してあげる
        if self.written_oam_data {
            self.written_oam_data = false;
            (false, true, self.ppu_reg[PPU_OAMDATA_OFFSET])
        } else if self.read_oam_data {
            self.read_oam_data = false;
            (true, false, self.ppu_reg[PPU_OAMDATA_OFFSET])
        } else {
            (false, false, self.ppu_reg[PPU_OAMDATA_OFFSET])
        }
    }

    pub fn write_oam_data(&mut self, data: u8) {
        self.ppu_reg[PPU_OAMDATA_OFFSET] = data;
    }

    /*************************** 0x2005: PPUSCROLL ***************************/
    /// (x,y更新があったか示すフラグ, x, y)
    pub fn read_ppu_scroll(&mut self) -> (bool, u8, u8) {
        if self.written_ppu_scroll {
            self.written_ppu_scroll = false;
            (true, self.ppu_reg[PPU_SCROLL_OFFSET], self.ppu_scroll_y_reg)
        } else {
            (
                false,
                self.ppu_reg[PPU_SCROLL_OFFSET],
                self.ppu_scroll_y_reg,
            )
        }
    }
    /*************************** 0x2006: PPUADDR ***************************/
    pub fn read_ppu_addr(&mut self) -> (bool, u16) {
        let addr =
            (u16::from(self.ppu_reg[PPU_ADDR_OFFSET]) << 8) | u16::from(self.ppu_addr_lower_reg);
        if self.written_ppu_addr {
            self.written_ppu_addr = false;
            (true, addr)
        } else {
            (false, addr)
        }
    }
    /*************************** 0x2007: PPUDATA ***************************/
    /// is_read, is_write, dataが返ります
    /// read/writeが同時にtrueにはならない。
    /// read : PPU_DATAにPPU_ADDRが示す値を非破壊で入れてあげ、アドレスインクリメント(自ずとpost-fetchになる)
    /// write: PPU_DATAの値をPPU_ADDR(PPU空間)に代入、アドレスインクリメント
    pub fn read_ppu_data(&mut self) -> (bool, bool, u8) {
        // Write優先でフラグ管理して返してあげる
        if self.written_ppu_data {
            self.written_ppu_data = false;
            (false, true, self.ppu_reg[PPU_DATA_OFFSET])
        } else if self.read_ppu_data {
            self.read_ppu_data = false;
            (true, false, self.ppu_reg[PPU_DATA_OFFSET])
        } else {
            (false, false, self.ppu_reg[PPU_DATA_OFFSET])
        }
    }

    /// 書き換えるけどオートインクリメントなどはしません
    pub fn write_ppu_data(&mut self, data: u8) {
        self.ppu_reg[PPU_DATA_OFFSET] = data;
    }

    /// PPU_DATAに読み書きをしたときのPPU_ADDR自動加算を行います
    pub fn increment_ppu_addr(&mut self) {
        let current_addr =
            (u16::from(self.ppu_reg[PPU_ADDR_OFFSET]) << 8) | u16::from(self.ppu_addr_lower_reg);
        // PPU_CTRLのPPU Addr Incrementに従う
        let add_val = u16::from(self.read_ppu_addr_increment());
        let dst_addr = current_addr.wrapping_add(add_val);
        // 分解して入れておく
        self.ppu_addr_lower_reg = (dst_addr & 0xff) as u8;
        self.ppu_reg[PPU_ADDR_OFFSET] = (dst_addr >> 8) as u8;
    }
    /*************************** 0x4014: OAM_DMA ***************************/
    /// DMA開始が必要かどうかと、転送元アドレスを返す
    /// 面倒なので読み取ったらtriggerは揮発させる
    pub fn read_oam_dma(&mut self) -> (bool, u16) {
        let start_addr = u16::from(self.io_reg[APU_IO_OAM_DMA_OFFSET]) << 8;
        if self.written_oam_dma {
            self.written_oam_dma = false;
            (true, start_addr)
        } else {
            (false, start_addr)
        }
    }
}
