/// Ram Size in bytes (4K) - Chip 8 has 4K of memory
pub const RAM_SIZE: usize = 4096;

/// Screen Size in bytes (64 x 32) - Chip 8 has a 64 x 32 pixel screen
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

/// Chip 8 has 16 8-bit registers
pub const NUM_REGS: usize = 16;

/// Chip 8 has a small stack which is an array of 16 16-bit values
const STACK_SIZE: usize = 16;

/// Chip 8 has 16 keys
const NUM_KEYS: usize = 16;

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emu {
    /// It is Chip-8 standard that the beginning of all Chip-8 programs will
    /// be loaded in starting at RAM address 0x200
    const START_ADDR: u16 = 0x200;
    pub fn new() -> Self {
        Self {
            pc: Self::START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emulator_starts_at_0x200() {
        let result = Emu::new();
        assert_eq!(result.pc, 0x200);
    }
}
