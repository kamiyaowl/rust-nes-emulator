use super::interface::{SystemBus, EmulateControl};
use super::cpu::Cpu;
use super::system::System;

#[derive(Copy, Clone)]
pub struct Position(u16, u16);

#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);
impl Color {
    /// 2C02の色情報をRGBに変換します
    /// ..VV_HHHH 形式
    /// V - 明度
    /// H - 色相
    fn from_2c02_fmt(src: u8) -> Color {
        let index = src & 0x3f;
        let table: [Color; 0x40] = include!("ppu_palette_table.rs");
        table[index as usize]
    }
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