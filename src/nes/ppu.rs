use super::interface::*;
use super::cassette::*;
use super::cpu::*;
use super::system::*;
use super::video_system::*;

/// 1lineあたりかかるCPUサイクル
pub const CPU_CYCLE_PER_LINE: usize = 341;

pub const VISIBLE_SCREEN_WIDTH  : usize = 256;
pub const VISIBLE_SCREEN_HEIGHT : usize = 240;

pub const RENDER_SCREEN_WIDTH   : u16 = 256;
pub const RENDER_SCREEN_HEIGHT  : u16 = 262; // 0 ~ 261

pub const OAM_SIZE                       : usize = 0x100; // dmaの転送サイズ
pub const OAM_DMA_COPY_SIZE_PER_PPU_STEP : u8   = 0xaa; // 341cyc/513cyc*256byte=170.1byte

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
    /// Object Attribute Memoryの実態
    pub oam: [u8; OAM_SIZE],
    /// DMAが稼働中か示す
    /// DMAには513cycかかるが、Emulation上ppuのstep2回341cyc*2で完了するので実行中フラグで処理する
    /// 先頭でDMA開始されたとして、前半341cycで67%(170byte/256byte)処理できる(ので、次のstepで残りを処理したら次のDMA要求を受けても行ける)
    pub is_dma_running: bool,
    /// DMAのCPU側のベースアドレス。ページ指定なのでlower byteは0
    pub dma_cpu_src_addr: u16, 
    /// DMAのOAM側のベースアドレス。256byteしたらwrapする(あまり使われないらしい)
    pub dma_oam_dst_addr: u8,
    /// 次処理するy_index
    pub current_line: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; OAM_SIZE],
            is_dma_running: false,
            dma_cpu_src_addr: 0,
            dma_oam_dst_addr: 0,
            current_line: 261,
        }
    }
}

impl EmulateControl for Ppu {
    fn reset(&mut self) {
        self.oam = [0; OAM_SIZE];
        self.is_dma_running = false;
        self.dma_cpu_src_addr = 0;
        self.dma_oam_dst_addr = 0;
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
    /// DMA転送を(2回に分けて)行います
    /// `is_pre_transfer` - 受領直後の転送ならtrue, ppu 1stepあとならfalse
    fn run_dma(&mut self, system: &mut System, is_pre_transfer: bool) {
        debug_assert!((!self.is_dma_running &&  is_pre_transfer) ||
                      ( self.is_dma_running && !is_pre_transfer));
        debug_assert!((self.dma_cpu_src_addr & 0x00ff) == 0x0000);

        // address計算
        let start_offset  : u8  = if is_pre_transfer { 0 } else { OAM_DMA_COPY_SIZE_PER_PPU_STEP };
        let cpu_start_addr: u16 = self.dma_cpu_src_addr.wrapping_add(u16::from(start_offset));
        let oam_start_addr: u8  = self.dma_oam_dst_addr.wrapping_add(start_offset);
        // 転送サイズ
        let transfer_size : u16 = if is_pre_transfer { OAM_DMA_COPY_SIZE_PER_PPU_STEP as u16 } else { (OAM_SIZE as u16) - u16::from(OAM_DMA_COPY_SIZE_PER_PPU_STEP) };

        if cfg!(debug_assertions) && cfg!(not(no_std)) {
            println!("[ppu][dma][{}] start_offset:{}, transfer_size:{}, cpu_start_addr:{:04x}, oam_start_addr:{:02x}", if is_pre_transfer { "pre " } else { "post" }, start_offset, transfer_size, cpu_start_addr, oam_start_addr);
        }

        // 転送
        for offset in 0..transfer_size {
            let cpu_addr = cpu_start_addr.wrapping_add(offset);
            let oam_addr = usize::from(oam_start_addr.wrapping_add(offset as u8));

            let cpu_data = system.read_u8(cpu_addr, false);
            self.oam[oam_addr] = cpu_data;
        }

        // ステータス更新
        self.is_dma_running  = is_pre_transfer;
    }
    
    /// PPUの処理を進めます(341 cpu cycleかかります)
    /// `cpu` - Interruptの要求が必要
    /// `system` - レジスタ読み書きする
    /// `video_system` - レジスタ読み書きする
    /// `cassette` - video_systemのアクセス領域に含まれてる
    /// `videoout_func` - pixelごとのデータが決まるごとに呼ぶ(NESは出力ダブルバッファとかない)
    pub fn step(&mut self, cpu: &mut Cpu, system: &mut System, video_system: &mut VideoSystem, cassette: &mut Cassette, fb: &mut [[Color; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT]) {
        if cfg!(debug_assertions) && cfg!(not(no_std)) {
            println!("[ppu][step] line:{}", self.current_line);
        }
        // PPU_SCROLL, PPU_ADDR, PPU_DATA読み書きに答えてあげる
        let (_, scroll_x, scroll_y) = system.read_ppu_scroll();
        let (_ , ppu_addr)          = system.read_ppu_addr();
        let (is_read_ppu_req, is_write_ppu_req, ppu_data) = system.read_ppu_data();
        if is_write_ppu_req {
            video_system.write_u8(cassette, ppu_addr, ppu_data);
            system.increment_ppu_addr();
            if cfg!(debug_assertions) && cfg!(not(no_std)) {
                println!("[ppu] cpu write_req addr:{:04x}, data:{:02x}", ppu_addr, ppu_data);
            }
        }
        if is_read_ppu_req {
            let data = video_system.read_u8(cassette, ppu_addr);
            system.write_ppu_data(data);
            system.increment_ppu_addr();
            if cfg!(debug_assertions) && cfg!(not(no_std)) {
                println!("[ppu] cpu read_req addr:{:04x}, data:{:02x}", ppu_addr, data);
            }
        }

        // OAM R/W (おおよそはDMAでやられるから使わないらしい)
        let oam_addr = system.read_ppu_oam_addr();
        let (is_read_oam_req, is_write_oam_req, oam_data) = system.read_oam_data();
        if is_write_oam_req {
            self.oam[usize::from(oam_addr)] = oam_data;
            if cfg!(debug_assertions) && cfg!(not(no_std)) {
                println!("[ppu][oam] cpu write_req addr:{:04x}, data:{:02x}", oam_addr, oam_data);
            }
        }
        if is_read_oam_req {
            let data = self.oam[usize::from(oam_addr)];
            system.write_oam_data(data);
            if cfg!(debug_assertions) && cfg!(not(no_std)) {
                println!("[ppu][oam] cpu read_req addr:{:04x}, data:{:02x}", oam_addr, data);
            }
        }

        // OAM DMA
        if self.is_dma_running {
            // 前回のOAM DMAのこりをやる
            self.run_dma(system, false);
        }
        let (is_dma_req, dma_cpu_src_addr) = system.read_oam_dma();
        if is_dma_req {
            // 新しいDMAのディスクリプタをセットして実行
            self.dma_cpu_src_addr = dma_cpu_src_addr;
            self.dma_oam_dst_addr = oam_addr;
            self.run_dma(system, true);
        }

        // 行の更新
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
                }
                // VBLANKフラグが立っていれば割り込みを発生させる($2002を読んでフラグをおろしてもらう)
                if system.read_ppu_nmi_enable() && system.read_ppu_is_vblank() {
                    cpu.interrupt(system, Interrupt::NMI);
                }

            },
            LineStatus::PreRender => {
                // VBLANKフラグを下ろす
                system.write_ppu_is_vblank(false);
            },
        };
        // 行カウンタを更新して終わり
        self.update_current_line();
    }

}