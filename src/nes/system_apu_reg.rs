use super::system::*;
use super::apu::*;

pub const APU_PULSE_1_OFFSET:        usize = 0x00;
pub const APU_PULSE_2_OFFSET:        usize = 0x04;
pub const APU_TRIANGLE_OFFSET:       usize = 0x08;
pub const APU_NOISE_OFFSET:          usize = 0x0c;
pub const APU_DMC_OFFSET:            usize = 0x10;
pub const APU_STATUS_OFFSET:         usize = 0x15;
pub const APU_FRAMECOUNTER_OFFSET:   usize = 0x15;

// APU & I/O(PAD) Register Implement
impl System {
    
}
