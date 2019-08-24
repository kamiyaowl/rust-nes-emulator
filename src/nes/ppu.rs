use super::interface::*;
use super::cpu::*;
use super::system::*;
use super::video_system::*;

/// 1lineあたりかかるCPUサイクル
pub const CPU_CYCLE_PER_LINE: usize = 341;

pub const VISIBLE_SCREEN_WIDTH  : usize = 256;
pub const VISIBLE_SCREEN_HEIGHT : usize = 240;

pub const RENDER_SCREEN_WIDTH   : u16 = 256;
pub const RENDER_SCREEN_HEIGHT  : u16 = 261;

#[derive(Copy, Clone)]
pub struct Position(pub u8, pub u8);

#[derive(Copy, Clone)]
/// R,G,B
pub struct Color(pub u8, pub u8, pub u8);
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
enum LineStatus {
    Visible, // 0~239
    PostRender, // 240
    VerticalBlanking(bool), // 241~260
    PreRender, // 261
}

impl LineStatus {
    fn from(line: u16) -> LineStatus {
        if line < 240 {
            LineStatus::Visible
        } else if line == 240 {
            LineStatus::PostRender
        } else if line < 261 {
            LineStatus::VerticalBlanking(line == 241)
        } else if line == 261 {
            LineStatus::PreRender
        } else {
            panic!("invalid line status");
        }
    }
}


pub struct Ppu {
    /// 次処理するy_index
    pub current_line: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            current_line: 261,
        }
    }
}

impl EmulateControl for Ppu {
    fn reset(&mut self) {
        self.current_line = 261;
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


impl Ppu {
    /// 0~239はvisible scanline
    /// 描画処理が必要
    fn update_current_line(&mut self) {
        self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
    }
    
    /// PPUの処理を進めます(341 cpu cycleかかります)
    /// `cpu` - Interruptの要求が必要
    /// `system` - レジスタ読み書きする
    /// `video_system` - レジスタ読み書きする
    /// `videoout_func` - pixelごとのデータが決まるごとに呼ぶ(NESは出力ダブルバッファとかない)
    pub fn step(&mut self, cpu: &mut Cpu, system: &mut System, video_system: &mut VideoSystem, fb: &mut [[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]) {
        if cfg!(debug_assertions) && cfg!(not(no_std)) {
            println!("[ppu][step] line:{}", self.current_line);
        }
        match LineStatus::from(self.current_line) {
            LineStatus::Visible => {
                // 1行描く
                for i in 0..VISIBLE_SCREEN_WIDTH {
                    // TEST
                    fb[usize::from(self.current_line)][i] = Color(i as u8, self.current_line as u8, (i as u8).wrapping_add(self.current_line as u8));
                }
            },
            LineStatus::PostRender => {
                // 何もしない
            },
            LineStatus::VerticalBlanking(is_first) => {
                // Line:241ならVBLANKフラグを立てる, NMI割り込み許可あればやる
                if is_first {
                    system.write_ppu_is_vblank(true);
                    if system.read_ppu_nmi_enable() {
                        cpu.interrupt(system, Interrupt::NMI);
                    }
                }
            },
            LineStatus::PreRender => {
                // VBLANKフラグを下ろす
                system.write_ppu_is_vblank(false);
            },
        };
        self.update_current_line();
    }

}