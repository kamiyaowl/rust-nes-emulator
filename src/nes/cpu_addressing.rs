use super::cpu::*;
use super::system::System;
use super::interface::{SystemBus};

/// Fetch and Adressing Implementation
/// Accumulatorとimplicitは実装の必要なし
/// 戻り地 (引いたアドレス, 条件分岐等で余計にかかるclock cycle)
impl Cpu {
    /// #v 即値をそのまま帰す
    pub fn addressing_immediate(&self, _system: &System, base_addr: u16) -> (u16, u8) {
        (base_addr, 0)
    }
    /// a
    pub fn addressing_absolute(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr = base_addr;
        let upper_addr = base_addr + 1;
        let lower = system.read_u8(lower_addr);
        let upper = system.read_u8(upper_addr);
        let addr  = (lower as u16) | ((upper as u16) << 8);
        (addr, 0)
    }
    /// (a) for JMP
    /// absolute indirect
    pub fn addressing_indirect(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr1 = base_addr;
        let upper_addr1 = base_addr + 1;
        let lower1 = system.read_u8(lower_addr1);
        let upper1 = system.read_u8(upper_addr1);
        let lower_addr2 = (lower1 as u16) | ((upper1 as u16) << 8);
        let upper_addr2 = lower_addr2.wrapping_add(1);
        let lower3 = system.read_u8(lower_addr2);
        let upper3 = system.read_u8(upper_addr2);
        let addr3 = (lower3 as u16) | ((upper3 as u16) << 8);
        (addr3, 0)
    }
    /// d
    pub fn addressing_zero_page(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr = base_addr;
        let lower = system.read_u8(lower_addr);
        let addr  = lower as u16;
        (addr, 0)
    }
    /// d,x
    pub fn addressing_zero_page_x(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr = base_addr;
        let lower = system.read_u8(lower_addr);
        let addr  = (lower as u16).wrapping_add(self.x as u16);
        (addr, 0)
    }
    /// d,y
    pub fn addressing_zero_page_y(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr = base_addr;
        let lower = system.read_u8(lower_addr);
        let addr  = (lower as u16).wrapping_add(self.y as u16);
        (addr, 0)
    }
    /// a,x
    pub fn addressing_absolute_x(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr = base_addr;
        let upper_addr = base_addr + 1;
        let lower = system.read_u8(lower_addr);
        let upper = system.read_u8(upper_addr);
        let addr  = ((lower as u16) | ((upper as u16) << 8)).wrapping_add(self.x as u16);

        let additional_cycle = if (base_addr & 0xff00u16) != (addr & 0xff00u16) { 1 } else { 0 };
        (addr, additional_cycle)
    }
    /// a,y
    pub fn addressing_absolute_y(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let lower_addr = base_addr;
        let upper_addr = base_addr + 1;
        let lower = system.read_u8(lower_addr);
        let upper = system.read_u8(upper_addr);
        let addr  = ((lower as u16) | ((upper as u16) << 8)).wrapping_add(self.y as u16);

        let additional_cycle = if (base_addr & 0xff00u16) != (addr & 0xff00u16) { 1 } else { 0 };
        (addr, additional_cycle)
    }
    /// label
    pub fn addressing_relative(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let offset_addr = base_addr;
        let offset = system.read_u8(offset_addr);
        let addr_signed  = ((offset as i8) as i32) + (self.pc as i32);
        debug_assert!(addr_signed >= 0);
        debug_assert!(addr_signed < 0x10000);
        let addr = addr_signed as u16 + 1; // 戻り先は指定+1にあるっぽい

        let additional_cycle = if (base_addr & 0xff00u16) != (addr & 0xff00u16) { 1 } else { 0 };
        (addr, additional_cycle)
    }
    /// (d,x)
    /// Indexed Indirect
    pub fn addressing_indirect_x(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let addr1 = base_addr;
        let data1 = system.read_u8(addr1);
        let addr2 = (data1 as u16).wrapping_add(self.x as u16);
        let data2_lower = system.read_u8(addr2);
        let data2_upper = system.read_u8(addr2.wrapping_add(1));
        let addr3 = (data2_lower as u16) | ((data2_upper as u16) << 8);
        (addr3, 0)
    }
    /// (d),y
    /// Indirect Indexed
    pub fn addressing_indirect_y(&self, system: &System, base_addr: u16) -> (u16, u8) {
        let addr1_lower = base_addr;
        let addr1_upper = self.pc.wrapping_add(2);
        let data1_lower = system.read_u8(addr1_lower);
        let data1_upper = system.read_u8(addr1_upper);
        let addr = ((data1_lower as u16) | ((data1_upper as u16) << 8)) + (self.y as u16);


        let additional_cycle = if (base_addr & 0xff00u16) != (addr & 0xff00u16) { 1 } else { 0 };
        (addr, additional_cycle)
    }
}