use super::interface::{SystemBus, EmulateControl};
use super::cpu::Cpu;
use super::system::System;

#[derive(Copy, Clone)]
pub struct Position(u8, u8);

#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);
impl Color {
    /// 2C02の色情報をRGBに変換します
    /// ..VV_HHHH 形式
    /// V - 明度
    /// H - 色相
    pub fn from_2c02_fmt(src: u8) -> Color {
        let index = src & 0x3f;
        let table: [Color; 0x40] = include!("ppu_palette_table.rs");
        table[index as usize]
    }
}

/// sprite.tile_idのu8から変換する
#[derive(Copy, Clone)]
enum TileId {
    /// 8*8 spriteの場合
    Normal { id: u8 },
    /// 8*16 spriteの場合
    /// TTTTTTTP
    /// P - pattern table addr(0:0x0000, 1: 0x1000)
    /// T - Tile Id
    Large { 
        /// P
        pattern_table_addr: u16,
        /// 2*T
        upper_tile_id: u8,
        /// 2*T+1
        lower_tile_id: u8,
    }
}
impl TileId {
    fn normal(src: u8) -> TileId {
        TileId::Normal {
            id: src
        }
    }
    fn large(src: u8) -> TileId {
        TileId::Large {
            pattern_table_addr: (if (src & 0x01) == 0x01 { 0x1000 } else { 0x0000u16 }),
            upper_tile_id: src & 0xfe,
            lower_tile_id: (src & 0xfe) + 1,
        }
    }
}
/// 描画に必要な補足情報とか
/// VHP___CC
#[derive(Copy, Clone)]
struct SpriteAttr {
    /// V 垂直反転
    is_vert_flip: bool,
    /// H 垂直反転
    is_hor_flip: bool,
    /// P 描画優先度
    is_draw_front: bool,
    /// CC pattele指定(2bit)
    palette_id: u8,
}
impl SpriteAttr {
    fn from(src: u8) -> SpriteAttr {
        SpriteAttr {
            is_vert_flip  : (src & 0x80) == 0x80,
            is_hor_flip   : (src & 0x40) == 0x40,
            is_draw_front : (src & 0x20) == 0x20,
            palette_id    : (src & 0x03),
        }
    }
}

struct Sprite {
    ///  y座標
    /// 実際は+1した場所に表示する
    y: u8, 
    /// tile ID指定
    tile_id: TileId,
    /// 属性とか
    attr: SpriteAttr,
    /// x座標
    x: u8,
}
pub struct Ppu {
}

impl Default for Ppu {
    fn default() -> Self {
        Self {

        }
    }
}

impl Ppu {
    pub fn step(cpu_cycles: usize, videoout_closure: impl Fn(Position, Color)) {
        unimplemented!();
    }

}