use super::cpu::*;
use super::interface::*;
use super::system::*;
use super::video_system::*;

/// 1lineあたりかかるCPUサイクル
pub const CPU_CYCLE_PER_LINE: usize = (341 / 3); // ppu cyc -> cpu cyc
/// 色の種類(RGB)
pub const NUM_OF_COLOR: usize = 3;
/// ユーザーに表示される領域幅
pub const VISIBLE_SCREEN_WIDTH: usize = 256;
/// ユーザーに表示される領域高さ
pub const VISIBLE_SCREEN_HEIGHT: usize = 240;
/// 実際に描画する幅(これは表示幅に等しい)
pub const RENDER_SCREEN_WIDTH: u16 = VISIBLE_SCREEN_WIDTH as u16;
/// VBlank期間を考慮した描画領域高さ
pub const RENDER_SCREEN_HEIGHT: u16 = 262; // 0 ~ 261
/// 1tileあたりのpixel数
pub const PIXEL_PER_TILE: u16 = 8; // 1tile=8*8
/// 横タイル数 32
pub const SCREEN_TILE_WIDTH: u16 = (VISIBLE_SCREEN_WIDTH as u16) / PIXEL_PER_TILE; // 256/8=32
/// 縦タイル数 30
pub const SCREEN_TILE_HEIGHT: u16 = (VISIBLE_SCREEN_HEIGHT as u16) / PIXEL_PER_TILE; // 240/8=30
/// 1属性テーブルエントリに対する横, 縦タイル数
pub const BG_NUM_OF_TILE_PER_ATTRIBUTE_TABLE_ENTRY: u16 = 4;
/// 属性テーブルの横エントリ数 8
pub const ATTRIBUTE_TABLE_WIDTH: u16 =
    (SCREEN_TILE_WIDTH / BG_NUM_OF_TILE_PER_ATTRIBUTE_TABLE_ENTRY);

/// PPU内部のOAMの容量 dmaの転送サイズと等しい
pub const OAM_SIZE: usize = 0x100;
/// DMA転送を2line処理で終えようと思ったときの1回目で転送するバイト数
/// 341cyc/513cyc*256byte=170.1byte
pub const OAM_DMA_COPY_SIZE_PER_PPU_STEP: u8 = 0xaa;
/// pattern1個あたりのエントリサイズ
pub const PATTERN_TABLE_ENTRY_BYTE: u16 = 16;

/// スプライトテンポラリレジスタ数
pub const SPRITE_TEMP_SIZE: usize = 8;
/// スプライト総数
pub const NUM_OF_SPRITE: usize = 64;
/// スプライト1個あたり4byte
pub const SPRITE_SIZE: usize = 4;
/// スプライトの横幅
pub const SPRITE_WIDTH: usize = 8;
pub const SPRITE_NORMAL_HEIGHT: usize = 8;
pub const SPRITE_LARGE_HEIGHT: usize = 16;
/// 1frame書くのにかかるサイクル数
pub const CYCLE_PER_DRAW_FRAME: usize = CPU_CYCLE_PER_LINE * ((RENDER_SCREEN_HEIGHT + 1) as usize);

#[derive(Copy, Clone)]
pub struct Position(pub u8, pub u8);

#[derive(Copy, Clone, Eq, PartialEq)]
/// R,G,B
pub struct Color(pub u8, pub u8, pub u8);
impl Color {
    /// 2C02の色情報をRGBに変換します
    /// ..VV_HHHH 形式
    /// V - 明度
    /// H - 色相
    pub fn from(src: u8) -> Color {
        let index = src & 0x3f;
        let table: [Color; 0x40] = include!("ppu_palette_table.rs");
        table[index as usize]
    }
    pub fn is_black(&self) -> bool {
        self.0 == 0x0 && self.1 == 0x0 && self.2 == 0x0
    }
}

/// sprite.tile_idのu8から変換する
#[derive(Copy, Clone)]
pub enum TileId {
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
    },
}
impl TileId {
    pub fn normal(src: u8) -> TileId {
        TileId::Normal { id: src }
    }
    pub fn large(src: u8) -> TileId {
        TileId::Large {
            pattern_table_addr: (if (src & 0x01) == 0x01 {
                0x1000
            } else {
                0x0000u16
            }),
            upper_tile_id: src & 0xfe,
            lower_tile_id: (src & 0xfe) + 1,
        }
    }
}
/// 描画に必要な補足情報とか
/// VHP___CC
#[derive(Copy, Clone)]
pub struct SpriteAttr {
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
    pub fn from(src: u8) -> SpriteAttr {
        SpriteAttr {
            is_vert_flip: (src & 0x80) == 0x80,
            is_hor_flip: (src & 0x40) == 0x40,
            is_draw_front: (src & 0x20) != 0x20,
            palette_id: (src & 0x03),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sprite {
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

impl Sprite {
    /// SpriteをOAMの情報から生成します。
    /// `is_large` -スプライトサイズが8*16ならtrue、8*8ならfalse
    pub fn from(is_large: bool, byte0: u8, byte1: u8, byte2: u8, byte3: u8) -> Sprite {
        Sprite {
            y: byte0,
            tile_id: (if is_large {
                TileId::large(byte1)
            } else {
                TileId::normal(byte1)
            }),
            attr: SpriteAttr::from(byte2),
            x: byte3,
        }
    }
}

#[derive(Copy, Clone)]
enum LineStatus {
    Visible,                // 0~239
    PostRender,             // 240
    VerticalBlanking(bool), // 241~260
    PreRender,              // 261
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

#[derive(Clone)]
pub struct Ppu {
    /// Object Attribute Memoryの実態
    pub oam: [u8; OAM_SIZE],
    /// 次の描画で使うスプライトを格納する
    pub sprite_temps: [Option<Sprite>; SPRITE_TEMP_SIZE],

    /// 積もり積もったcpu cycle, 341を超えたらクリアして1行処理しよう
    pub cumulative_cpu_cyc: usize,
    /// 次処理するy_index
    pub current_line: u16,

    // scrollレジスタは1lineごとに更新
    pub fetch_scroll_x: u8,
    pub fetch_scroll_y: u8,
    pub current_scroll_x: u8,
    pub current_scroll_y: u8,

    /// DMAが稼働中か示す
    /// DMAには513cycかかるが、Emulation上ppuのstep2回341cyc*2で完了するので実行中フラグで処理する
    /// 先頭でDMA開始されたとして、前半341cycで67%(170byte/256byte)処理できる(ので、次のstepで残りを処理したら次のDMA要求を受けても行ける)
    pub is_dma_running: bool,
    /// DMAのCPU側のベースアドレス。ページ指定なのでlower byteは0
    pub dma_cpu_src_addr: u16,
    /// DMAのOAM側のベースアドレス。256byteしたらwrapする(あまり使われないらしい)
    pub dma_oam_dst_addr: u8,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; OAM_SIZE],
            sprite_temps: [None; SPRITE_TEMP_SIZE],

            cumulative_cpu_cyc: 0,
            current_line: 241,

            fetch_scroll_x: 0,
            fetch_scroll_y: 0,
            current_scroll_x: 0,
            current_scroll_y: 0,

            is_dma_running: false,
            dma_cpu_src_addr: 0,
            dma_oam_dst_addr: 0,
        }
    }
}

impl EmulateControl for Ppu {
    fn reset(&mut self) {
        self.oam = [0; OAM_SIZE];
        self.sprite_temps = [None; SPRITE_TEMP_SIZE];

        self.current_line = 241;
        self.cumulative_cpu_cyc = 0;

        self.fetch_scroll_x = 0;
        self.fetch_scroll_y = 0;
        self.current_scroll_x = 0;
        self.current_scroll_y = 0;

        self.is_dma_running = false;
        self.dma_cpu_src_addr = 0;
        self.dma_oam_dst_addr = 0;
    }
}

impl Ppu {
    /// DMA転送を(2回に分けて)行います
    /// `is_pre_transfer` - 受領直後の転送ならtrue, ppu 1stepあとならfalse
    fn run_dma(&mut self, system: &mut System, is_pre_transfer: bool) {
        debug_assert!(
            (!self.is_dma_running && is_pre_transfer) || (self.is_dma_running && !is_pre_transfer)
        );
        debug_assert!((self.dma_cpu_src_addr & 0x00ff) == 0x0000);

        // address計算
        let start_offset: u8 = if is_pre_transfer {
            0
        } else {
            OAM_DMA_COPY_SIZE_PER_PPU_STEP
        };
        let cpu_start_addr: u16 = self.dma_cpu_src_addr.wrapping_add(u16::from(start_offset));
        let oam_start_addr: u8 = self.dma_oam_dst_addr.wrapping_add(start_offset);
        // 転送サイズ
        let transfer_size: u16 = if is_pre_transfer {
            OAM_DMA_COPY_SIZE_PER_PPU_STEP as u16
        } else {
            (OAM_SIZE as u16) - u16::from(OAM_DMA_COPY_SIZE_PER_PPU_STEP)
        };

        // 転送
        for offset in 0..transfer_size {
            let cpu_addr = cpu_start_addr.wrapping_add(offset);
            let oam_addr = usize::from(oam_start_addr.wrapping_add(offset as u8));

            let cpu_data = system.read_u8(cpu_addr, false);
            self.oam[oam_addr] = cpu_data;
        }

        // ステータス更新
        self.is_dma_running = is_pre_transfer;
    }
    /// 1行書きます
    ///
    /// `tile_base`   - スクロールオフセット加算なしの現在のタイル位置
    /// `tile_global` - スクロールオフセット換算した、4面含めた上でのタイル位置
    /// `tile_local`  - `tile_global`を1Namespace上のタイルでの位置に変換したもの
    /// scrollなしなら上記はすべて一致するはず
    fn draw_line(
        &mut self,
        system: &mut System,
        fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    ) {
        // ループ内で何度も呼び出すとパフォーマンスが下がる
        let nametable_base_addr = system.read_ppu_name_table_base_addr();
        let pattern_table_addr = system.read_ppu_bg_pattern_table_addr();
        let is_clip_bg_leftend = system.read_ppu_is_clip_bg_leftend();
        let is_write_bg = system.read_ppu_is_write_bg();
        let is_monochrome = system.read_is_monochrome();
        let master_bg_color = Color::from(system.video.read_u8(
            &mut system.cassette,
            PALETTE_TABLE_BASE_ADDR + PALETTE_BG_OFFSET,
        ));

        let raw_y = self.current_line + u16::from(self.current_scroll_y);
        let offset_y = raw_y & 0x07; // tile換算でのy位置から、実pixelのズレ(0~7)
        let tile_base_y = raw_y >> 3; // オフセットなしのtile換算での現在位置
                                      // scroll regはtile換算でずらす
        let tile_global_y = tile_base_y % (SCREEN_TILE_HEIGHT * 2); // tile換算でのy絶対座標
        let tile_local_y = tile_global_y % SCREEN_TILE_HEIGHT; // 1 tile内での絶対座標
                                                               // 4面ある内、下側に差し掛かっていたらfalse
        let is_nametable_position_top = tile_global_y < SCREEN_TILE_HEIGHT;

        // 描画座標系でループさせる
        let pixel_y = usize::from(self.current_line);
        for pixel_x in 0..VISIBLE_SCREEN_WIDTH {
            // Sprite: 探索したテンポラリレジスタから描画するデータを取得する
            let (sprite_palette_data_back, sprite_palette_data_front) =
                self.get_sprite_draw_data(system, pixel_x, pixel_y);

            // BG(Nametable): 座標に該当するNametableと属性テーブルからデータを取得する
            let offset_x = ((pixel_x as u16) + u16::from(self.current_scroll_x)) & 0x07;
            let tile_base_x = ((pixel_x as u16) + u16::from(self.current_scroll_x)) >> 3;
            // scroll regはtile換算でずらす
            let tile_global_x = tile_base_x % (SCREEN_TILE_WIDTH * 2); // 4tile換算でのx絶対座標
            let tile_local_x = tile_global_x % SCREEN_TILE_WIDTH; // 1 tile内での絶対座標
            let is_nametable_position_left = tile_global_x < SCREEN_TILE_WIDTH; // 4面ある内、右側にある場合false

            // 4面あるうちのどれかがわかるので、該当する面のベースアドレスを返します
            let target_nametable_base_addr = nametable_base_addr +
                (if is_nametable_position_left { 0x0000 } else { 0x0400 }) + // 左右面の広域offset
                (if is_nametable_position_top  { 0x0000 } else { 0x0800 }); // 上下面の広域offset

            // attribute tableはNametableの後32byteにいるのでアドレス計算して読み出す。縦横4*4tileで1attrになっている
            // scroll対応のためにoffset計算はglobal位置を使っている（もしかしたら1Nametableでクリッピングがいるかも)
            let attribute_base_addr = target_nametable_base_addr + ATTRIBUTE_TABLE_OFFSET; // 23c0, 27c0, 2bc0, 2fc0のどれか
            let attribute_x_offset = (tile_global_x >> 2) & 0x7;
            let attribute_y_offset = tile_global_y >> 2;
            let attribute_addr =
                attribute_base_addr + (attribute_y_offset << 3) + attribute_x_offset;

            // attribute読み出し, BGパレット選択に使う。4*4の位置で使うパレット情報を変える
            let raw_attribute = system.video.read_u8(&mut system.cassette, attribute_addr);
            let bg_palette_id = match (tile_local_x & 0x03 < 0x2, tile_local_y & 0x03 < 0x2) {
                (true, true) => (raw_attribute >> 0) & 0x03,  // top left
                (false, true) => (raw_attribute >> 2) & 0x03, // top right
                (true, false) => (raw_attribute >> 4) & 0x03, // bottom left
                (false, false) => (raw_attribute >> 6) & 0x03, // bottom right
            };

            // Nametableからtile_id読み出し->pattern tableからデータ構築
            let nametable_addr = target_nametable_base_addr + (tile_local_y << 5) + tile_local_x;
            let bg_tile_id = u16::from(system.video.read_u8(&mut system.cassette, nametable_addr));

            // pattern_table 1entryは16byte, 0行目だったら0,8番目のデータを使えば良い
            let bg_pattern_table_base_addr = pattern_table_addr + (bg_tile_id << 4);
            let bg_pattern_table_addr_lower = bg_pattern_table_base_addr + offset_y;
            let bg_pattern_table_addr_upper = bg_pattern_table_addr_lower + 8;
            let bg_data_lower = system
                .video
                .read_u8(&mut system.cassette, bg_pattern_table_addr_lower);
            let bg_data_upper = system
                .video
                .read_u8(&mut system.cassette, bg_pattern_table_addr_upper);

            // bgの描画色を作る
            let bg_palette_offset = (((bg_data_upper >> (7 - offset_x)) & 0x01) << 1)
                | ((bg_data_lower >> (7 - offset_x)) & 0x01);
            let bg_palette_addr = (PALETTE_TABLE_BASE_ADDR + PALETTE_BG_OFFSET) +   // 0x3f00
                (u16::from(bg_palette_id) << 2) + // attributeでBG Palette0~3選択
                u16::from(bg_palette_offset); // palette内の色選択

            // BG左端8pixel clipも考慮してBGデータ作る
            let is_bg_clipping = is_clip_bg_leftend && (pixel_x < 8);
            let is_bg_tranparent = (bg_palette_addr & 0x03) == 0x00; // 背景色が選択された場合はここで処理してしまう
            let bg_palette_data: Option<u8> = if is_bg_clipping || !is_write_bg || is_bg_tranparent
            {
                None
            } else {
                Some(system.video.read_u8(&mut system.cassette, bg_palette_addr))
            };

            // 透明色
            let mut draw_color = master_bg_color;

            // 前後関係考慮して書き込む
            'select_color: for palette_data in &[
                sprite_palette_data_front,
                bg_palette_data,
                sprite_palette_data_back,
            ] {
                // 透明色判定をしていたら事前にNoneされている
                if let Some(color_index) = palette_data {
                    let c = Color::from(*color_index);
                    draw_color = c;
                    break 'select_color;
                }
            }
            // データをFBに反映
            fb[pixel_y][pixel_x][0] = draw_color.0;
            fb[pixel_y][pixel_x][1] = draw_color.1;
            fb[pixel_y][pixel_x][2] = draw_color.2;

            // モノクロ出力対応(とりあえず総加平均...)
            if is_monochrome {
                let data = ((u16::from(fb[pixel_y][pixel_x][0])
                    + u16::from(fb[pixel_y][pixel_x][1])
                    + u16::from(fb[pixel_y][pixel_x][2]))
                    / 3) as u8;
                fb[pixel_y][pixel_x][0] = data;
                fb[pixel_y][pixel_x][1] = data;
                fb[pixel_y][pixel_x][2] = data;
            }
        }
    }
    /// 指定されたpixelにあるスプライトを描画します
    /// `pixel_x` - 描画対象の表示するリーンにおけるx座標
    /// `pixel_y` - 描画対象の表示するリーンにおけるy座標
    /// retval - (bgよりも後ろに描画するデータ, bgより前に描画するデータ)
    fn get_sprite_draw_data(
        &mut self,
        system: &mut System,
        pixel_x: usize,
        pixel_y: usize,
    ) -> (Option<u8>, Option<u8>) {
        // Sprite描画無効化されていたら即終了
        if !system.read_ppu_is_write_sprite() {
            return (None, None);
        }
        // Spriteを探索する (y位置的に描画しなければならないSpriteは事前に読み込み済)
        let mut sprite_palette_data_back: Option<u8> = None; // 背面
        let mut sprite_palette_data_front: Option<u8> = None; // 全面
        'draw_sprite: for &s in self.sprite_temps.iter() {
            if let Some(sprite) = s {
                // めんどいのでusizeにしておく
                let sprite_x = usize::from(sprite.x);
                let sprite_y = usize::from(sprite.y);
                // 左端sprite clippingが有効な場合表示しない
                let is_sprite_clipping = system.read_ppu_is_clip_sprite_leftend() && (pixel_x < 8);
                // X位置が描画範囲の場合
                if !is_sprite_clipping
                    && (sprite_x <= pixel_x)
                    && (pixel_x < usize::from(sprite_x + SPRITE_WIDTH))
                {
                    // sprite上での相対座標
                    let sprite_offset_x: usize = pixel_x - sprite_x; // 0-7
                    let sprite_offset_y: usize = pixel_y - sprite_y - 1; // 0-7 or 0-15 (largeの場合, tile参照前に0-7に詰める)
                    debug_assert!(sprite_offset_x < SPRITE_WIDTH);
                    debug_assert!(sprite_offset_y < usize::from(system.read_ppu_sprite_height()));
                    // pattern table addrと、tile idはサイズで決まる
                    let (sprite_pattern_table_addr, sprite_tile_id): (u16, u8) = match sprite
                        .tile_id
                    {
                        TileId::Normal { id } => (system.read_ppu_sprite_pattern_table_addr(), id),
                        // 8*16 spriteなので上下でidが別れている
                        TileId::Large {
                            pattern_table_addr,
                            upper_tile_id,
                            lower_tile_id,
                        } => {
                            let is_upper = sprite_offset_y < SPRITE_NORMAL_HEIGHT; // 上8pixelの座標?
                            let is_vflip = sprite.attr.is_vert_flip; // 上下反転してる?
                            let id = match (is_upper, is_vflip) {
                                (true, false) => upper_tile_id,  // 描画座標は上8pixel、Flipなし
                                (false, false) => lower_tile_id, // 描画座標は下8pixel、Flipなし
                                (true, true) => lower_tile_id,   // 描画座標は上8pixel、Flipあり
                                (false, true) => upper_tile_id,  // 描画座標は下8pixel、Flipあり
                            };
                            (pattern_table_addr, id)
                        }
                    };
                    // x,y flipを考慮してtile上のデータ位置を決定する
                    let tile_offset_x: usize = if !sprite.attr.is_hor_flip {
                        sprite_offset_x
                    } else {
                        SPRITE_WIDTH - 1 - sprite_offset_x
                    };
                    let tile_offset_y: usize = if !sprite.attr.is_vert_flip {
                        sprite_offset_y % SPRITE_NORMAL_HEIGHT
                    } else {
                        SPRITE_NORMAL_HEIGHT - 1 - (sprite_offset_y % SPRITE_NORMAL_HEIGHT)
                    };
                    // tile addrを計算する
                    let sprite_pattern_table_base_addr = u16::from(sprite_pattern_table_addr)
                        + (u16::from(sprite_tile_id) * PATTERN_TABLE_ENTRY_BYTE);
                    let sprite_pattern_table_addr_lower =
                        sprite_pattern_table_base_addr + (tile_offset_y as u16);
                    let sprite_pattern_table_addr_upper = sprite_pattern_table_addr_lower + 8;
                    let sprite_data_lower = system
                        .video
                        .read_u8(&mut system.cassette, sprite_pattern_table_addr_lower);
                    let sprite_data_upper = system
                        .video
                        .read_u8(&mut system.cassette, sprite_pattern_table_addr_upper);
                    // 該当するx位置のpixel patternを作る
                    let sprite_palette_offset =
                        (((sprite_data_upper >> (7 - tile_offset_x)) & 0x01) << 1)
                            | ((sprite_data_lower >> (7 - tile_offset_x)) & 0x01);
                    // paletteのアドレスを計算する
                    let sprite_palette_addr = (PALETTE_TABLE_BASE_ADDR + PALETTE_SPRITE_OFFSET) +        // 0x3f10
                        (u16::from(sprite.attr.palette_id) * PALETTE_ENTRY_SIZE) + // attributeでSprite Palette0~3選択
                        u16::from(sprite_palette_offset); // palette内の色選択
                                                          // パレットが透明色の場合はこのpixelは描画しない
                    let is_tranparent = (sprite_palette_addr & 0x03) == 0x00; // 背景色が選択された
                    if !is_tranparent {
                        // パレットを読み出し
                        let sprite_palette_data = system
                            .video
                            .read_u8(&mut system.cassette, sprite_palette_addr);
                        // 表裏の優先度がattrにあるので、該当する方に書き込み
                        if sprite.attr.is_draw_front {
                            sprite_palette_data_front = Some(sprite_palette_data);
                        } else {
                            sprite_palette_data_back = Some(sprite_palette_data);
                        }
                    }
                }
            } else {
                // sprite tempsは前詰めなのでもう処理はいらない
                break 'draw_sprite;
            }
        }
        // 描画するデータを返す
        (sprite_palette_data_back, sprite_palette_data_front)
    }

    /// OAMを探索して次の描画で使うスプライトをレジスタにフェッチします
    /// 8個を超えるとOverflowフラグを立てる
    fn fetch_sprite(&mut self, system: &mut System) {
        // sprite描画無効化
        if !system.read_ppu_is_write_sprite() {
            return;
        }
        // スプライトのサイズを事前計算
        let sprite_begin_y = self.current_line;
        let sprite_height = u16::from(system.read_ppu_sprite_height());
        let is_large = sprite_height == 16;
        // とりあえず全部クリアしておく
        self.sprite_temps = [None; SPRITE_TEMP_SIZE];
        // current_line + 1がyと一致するやつを順番に集める(条件分がよりでかいにしてある)
        let mut tmp_index = 0;
        'search_sprite: for sprite_index in 0..NUM_OF_SPRITE {
            let target_oam_addr = sprite_index << 2;
            // yの値と等しい
            let sprite_y = u16::from(self.oam[target_oam_addr]);
            let sprite_end_y = sprite_y + sprite_height;
            // 描画範囲内(y+1)~(y+1+ 8or16)
            if (sprite_y < sprite_begin_y) && (sprite_begin_y <= sprite_end_y) {
                // sprite 0 hitフラグ(1lineごとに処理しているので先に立ててしまう)
                let is_zero_hit_delay = sprite_begin_y > (sprite_end_y - 3); //1lineずつ処理だとマリオ等早く検知しすぎるので TODO: #40
                if sprite_index == 0 && is_zero_hit_delay {
                    system.write_ppu_is_hit_sprite0(true);
                }
                // sprite overflow
                if tmp_index >= SPRITE_TEMP_SIZE {
                    system.write_ppu_is_sprite_overflow(true);
                    break 'search_sprite;
                } else {
                    debug_assert!(tmp_index < SPRITE_TEMP_SIZE);
                    // tmp regに格納する
                    self.sprite_temps[tmp_index] = Some(Sprite::from(
                        is_large,
                        self.oam[target_oam_addr],
                        self.oam[target_oam_addr + 1],
                        self.oam[target_oam_addr + 2],
                        self.oam[target_oam_addr + 3],
                    ));
                    tmp_index = tmp_index + 1;
                }
            }
        }
    }

    /// 1行ごとに色々更新する処理です
    /// 341cyc溜まったときに呼び出されることを期待
    fn update_line(
        &mut self,
        system: &mut System,
        fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    ) -> Option<Interrupt> {
        // scroll更新
        self.current_scroll_x = self.fetch_scroll_x;
        self.current_scroll_y = self.fetch_scroll_y;
        // OAM DMA
        if self.is_dma_running {
            // 前回のOAM DMAのこりをやる
            self.run_dma(system, false);
        }
        let (is_dma_req, dma_cpu_src_addr) = system.read_oam_dma();
        if is_dma_req {
            // 新しいDMAのディスクリプタをセットして実行
            self.dma_cpu_src_addr = dma_cpu_src_addr;
            self.dma_oam_dst_addr = system.read_ppu_oam_addr();
            self.run_dma(system, true);
        }
        // ステータスを初期化
        system.write_ppu_is_hit_sprite0(false);
        system.write_ppu_is_sprite_overflow(false);

        // 行の更新
        match LineStatus::from(self.current_line) {
            LineStatus::Visible => {
                // sprite探索
                self.fetch_sprite(system);
                // 1行描く
                self.draw_line(system, fb);
                // 行カウンタを更新して終わり
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;

                None
            }
            LineStatus::PostRender => {
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
                None
            }
            LineStatus::VerticalBlanking(is_first) => {
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
                if is_first {
                    system.write_ppu_is_vblank(true);
                }
                // VBLANKフラグが立っていれば割り込みを発生させる($2002を読んでフラグをおろしてもらう)
                if system.read_ppu_nmi_enable() && system.read_ppu_is_vblank() {
                    Some(Interrupt::NMI)
                } else {
                    None
                }
            }
            LineStatus::PreRender => {
                self.current_line = (self.current_line + 1) % RENDER_SCREEN_HEIGHT;
                // VBLANKフラグを下ろす
                system.write_ppu_is_vblank(false);

                None
            }
        }
    }

    /// PPUの処理を進めます(1line進めるまでには341 cpu cycleかかります)
    /// `cpu_cyc` - cpuが何clock処理したか入れる(cpu 1stepごとに呼ぶこと)
    /// `cpu` - Interruptの要求が必要
    /// `system` - レジスタ読み書きする
    /// `video_system` - レジスタ読み書きする
    /// `videoout_func` - pixelごとのデータが決まるごとに呼ぶ(NESは出力ダブルバッファとかない)
    pub fn step(
        &mut self,
        cpu_cyc: usize,
        system: &mut System,
        fb: &mut [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    ) -> Option<Interrupt> {
        // PPU_SCROLL書き込み
        let (_, scroll_x, scroll_y) = system.read_ppu_scroll();
        self.fetch_scroll_x = scroll_x;
        self.fetch_scroll_y = scroll_y;

        // PPU_ADDR, PPU_DATA読み書きに答えてあげる
        let (_, ppu_addr) = system.read_ppu_addr();
        let (is_read_ppu_req, is_write_ppu_req, ppu_data) = system.read_ppu_data();

        if is_write_ppu_req {
            system
                .video
                .write_u8(&mut system.cassette, ppu_addr, ppu_data);
            system.increment_ppu_addr();
        }
        if is_read_ppu_req {
            let data = system.video.read_u8(&mut system.cassette, ppu_addr);
            system.write_ppu_data(data);
            system.increment_ppu_addr();
        }

        // OAM R/W (おおよそはDMAでやられるから使わないらしい)
        let oam_addr = system.read_ppu_oam_addr();
        let (is_read_oam_req, is_write_oam_req, oam_data) = system.read_oam_data();
        if is_write_oam_req {
            self.oam[usize::from(oam_addr)] = oam_data;
        }
        if is_read_oam_req {
            let data = self.oam[usize::from(oam_addr)];
            system.write_oam_data(data);
        }

        // clock cycle判定して行更新
        let total_cyc = self.cumulative_cpu_cyc + cpu_cyc;
        if total_cyc >= CPU_CYCLE_PER_LINE {
            self.cumulative_cpu_cyc = total_cyc - CPU_CYCLE_PER_LINE;
            self.update_line(system, fb)
        } else {
            self.cumulative_cpu_cyc = total_cyc;
            None
        }
    }
}
