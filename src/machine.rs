use std::sync::{Arc, Mutex};

pub struct Machine {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
    pub dt: Arc<Mutex<u8>>,
    pub st: Arc<Mutex<u8>>,
    pub pc: u16,
    pub sp: u16,
    pub stack: [u16; 16],
    pub keys: [bool; 16],
    pub sprite_digits: [u16; 16],
    pub display: Arc<Mutex<[bool; 64 * 32]>>
}

impl Machine {
    pub fn init() -> Machine {
        let mut m = Machine {
            memory: [0u8; 4096],
            v: [0u8; 16],
            i: 0u16,
            dt: Arc::new(Mutex::new(0u8)),
            st: Arc::new(Mutex::new(0u8)),
            pc: 0u16,
            sp: 0u16,
            stack: [0u16; 16],
            keys: [false; 16],
            sprite_digits: [0u16; 16],
            display: Arc::new(Mutex::new([false; 64 * 32]))
        };

        m.sprite_digits[0x0] = 0u16;
        m.memory[0] = 0xF0u8;
        m.memory[1] = 0x90u8;
        m.memory[2] = 0x90u8;
        m.memory[3] = 0x90u8;
        m.memory[4] = 0xF0u8;

        m.sprite_digits[0x1] = 5u16;
        m.memory[5] = 0x20u8;
        m.memory[6] = 0x60u8;
        m.memory[7] = 0x20u8;
        m.memory[8] = 0x20u8;
        m.memory[9] = 0x70u8;

        m.sprite_digits[0x2] = 10u16;
        m.memory[10] = 0xF0u8;
        m.memory[11] = 0x10u8;
        m.memory[12] = 0xF0u8;
        m.memory[13] = 0x80u8;
        m.memory[14] = 0xF0u8;

        m.sprite_digits[0x3] = 15u16;
        m.memory[15] = 0xF0u8;
        m.memory[16] = 0x10u8;
        m.memory[17] = 0xF0u8;
        m.memory[18] = 0x10u8;
        m.memory[19] = 0xF0u8;

        m.sprite_digits[0x4] = 20u16;
        m.memory[20] = 0x90u8;
        m.memory[21] = 0x90u8;
        m.memory[22] = 0xF0u8;
        m.memory[23] = 0x10u8;
        m.memory[24] = 0x10u8;

        m.sprite_digits[0x5] = 25u16;
        m.memory[25] = 0xF0u8;
        m.memory[26] = 0x80u8;
        m.memory[27] = 0xF0u8;
        m.memory[28] = 0x10u8;
        m.memory[29] = 0xF0u8;

        m.sprite_digits[0x6] = 30u16;
        m.memory[30] = 0xF0u8;
        m.memory[31] = 0x80u8;
        m.memory[32] = 0xF0u8;
        m.memory[33] = 0x90u8;
        m.memory[34] = 0xF0u8;

        m.sprite_digits[0x7] = 35u16;
        m.memory[35] = 0xF0u8;
        m.memory[36] = 0x10u8;
        m.memory[37] = 0x20u8;
        m.memory[38] = 0x40u8;
        m.memory[39] = 0x40u8;

        m.sprite_digits[0x8] = 40u16;
        m.memory[40] = 0xF0u8;
        m.memory[41] = 0x90u8;
        m.memory[42] = 0xF0u8;
        m.memory[43] = 0x90u8;
        m.memory[44] = 0xF0u8;

        m.sprite_digits[0x9] = 45u16;
        m.memory[45] = 0xF0u8;
        m.memory[46] = 0x90u8;
        m.memory[47] = 0xF0u8;
        m.memory[48] = 0x10u8;
        m.memory[49] = 0xF0u8;

        m.sprite_digits[0xA] = 50u16;
        m.memory[50] = 0xF0u8;
        m.memory[51] = 0x90u8;
        m.memory[52] = 0xF0u8;
        m.memory[53] = 0x90u8;
        m.memory[54] = 0x90u8;

        m.sprite_digits[0xB] = 55u16;
        m.memory[55] = 0xE0u8;
        m.memory[56] = 0x90u8;
        m.memory[57] = 0xE0u8;
        m.memory[58] = 0x90u8;
        m.memory[59] = 0xE0u8;

        m.sprite_digits[0xC] = 60u16;
        m.memory[60] = 0xF0u8;
        m.memory[61] = 0x80u8;
        m.memory[62] = 0x80u8;
        m.memory[63] = 0x80u8;
        m.memory[64] = 0xF0u8;

        m.sprite_digits[0xD] = 65u16;
        m.memory[65] = 0xF0u8;
        m.memory[66] = 0x90u8;
        m.memory[67] = 0x90u8;
        m.memory[68] = 0x90u8;
        m.memory[69] = 0xF0u8;

        m.sprite_digits[0xE] = 70u16;
        m.memory[70] = 0xF0u8;
        m.memory[71] = 0x80u8;
        m.memory[72] = 0xF0u8;
        m.memory[73] = 0x80u8;
        m.memory[74] = 0xF0u8;

        m.sprite_digits[0xF] = 75u16;
        m.memory[75] = 0xF0u8;
        m.memory[76] = 0x80u8;
        m.memory[77] = 0xF0u8;
        m.memory[78] = 0x80u8;
        m.memory[79] = 0x80u8;

        return m;
    }
}