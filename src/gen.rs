use maikor_platform::mem::address::{ATLAS1, IRQ_CONTROL, PALETTES, SPRITE_TABLE};
use maikor_platform::mem::{address, interrupt_flags};
use maikor_platform::models::{Byteable, Sprite};
use maikor_platform::ops::{
    CMP_REG_ADDR_BYTE, CMP_REG_NUM_BYTE, CPY_ADDR_NUM_BYTE, CPY_REG_ADDR_BYTE, INC_ADDR_BYTE,
    JE_ADDR, JMP_ADDR, MEM_CPY_ADDR_ADDR_BYTE, RETI,
};
use maikor_platform::registers::id;
use maikor_vm_file::{GameFile, GameFileHeader};
use nanorand::Rng;

pub fn game() -> GameFile {
    GameFile::new(
        GameFileHeader::new(
            "test".to_string(),
            1,
            1,
            1,
            2,
            "test".to_string(),
            "v1".to_string(),
            "Ray".to_string(),
            0,
            1,
            9,
        ),
        code(),
        vec![],
        vec![altas()],
        controller(),
    )
}

fn code() -> [u8; 9000] {
    let mut output = [0; 9000];

    let palette = PALETTES.to_be_bytes();
    let atlas = ATLAS1.to_be_bytes();
    let sprite_table = SPRITE_TABLE.to_be_bytes();
    let control = IRQ_CONTROL.to_be_bytes();

    let ops = vec![
        MEM_CPY_ADDR_ADDR_BYTE,
        palette[0],
        palette[1],
        0x17,
        0x70,
        16 * 3,
        MEM_CPY_ADDR_ADDR_BYTE,
        atlas[0],
        atlas[1],
        0x1b,
        0x58,
        200,
        MEM_CPY_ADDR_ADDR_BYTE,
        sprite_table[0],
        sprite_table[1],
        0x18,
        0x38,
        5,
        CPY_ADDR_NUM_BYTE,
        control[0],
        control[1],
        255,
        CPY_ADDR_NUM_BYTE,
        0x88,
        0x08,
        0x80,
        CPY_ADDR_NUM_BYTE,
        0x88,
        0x06,
        0x77,
        CPY_ADDR_NUM_BYTE,
        0x88,
        0x07,
        0xFF,
    ];

    output[50] = INC_ADDR_BYTE;
    output[51] = 0xd7;
    output[52] = 0x51;
    output[53] = JMP_ADDR;
    output[55] = 50;

    output[address::interrupt::IRQ_INPUT as usize] = JMP_ADDR;
    output[address::interrupt::IRQ_INPUT as usize + 1] = 0x03;
    output[address::interrupt::IRQ_INPUT as usize + 2] = 0xe8;

    for (i, value) in ops.iter().enumerate() {
        output[i] = *value;
    }

    let input_bytes = vec![
        CPY_ADDR_NUM_BYTE,
        0xd7,
        0x52,
        100,
        CPY_REG_ADDR_BYTE,
        id::BH,
        address::INPUT.to_be_bytes()[0],
        address::INPUT.to_be_bytes()[1],
        CMP_REG_NUM_BYTE,
        id::BH,
        0,
        JE_ADDR,
        4,
        10,
        CPY_ADDR_NUM_BYTE,
        0x87,
        0xf2,
        0x16,
        CPY_ADDR_NUM_BYTE,
        0x87,
        0xf3,
        0x40,
        CPY_ADDR_NUM_BYTE,
        0x87,
        0xf4,
        0x73,
        CPY_ADDR_NUM_BYTE,
        0x87,
        0xf5,
        0x0,
        CPY_ADDR_NUM_BYTE,
        0x87,
        0xf6,
        0xc3,
        RETI,
    ];

    for (i, value) in input_bytes.iter().enumerate() {
        output[i + 1000] = *value;
    }

    let palette_data: [u8; 3 * 16] = [
        230, 20, 20, 20, 230, 20, 20, 20, 230, 230, 230, 230, 60, 60, 150, 150, 60, 60, 60, 150,
        60, 180, 90, 150, 150, 180, 90, 90, 150, 180, 20, 140, 230, 230, 20, 140, 140, 230, 20,
        230, 230, 20, 140, 20, 20, 20, 20, 20,
    ];

    for (i, value) in palette_data.iter().enumerate() {
        output[i + 6000] = *value;
    }

    let mut table_data = vec![];
    let sprite1 = Sprite::new(9, 9, 0, false, false, 0, false, 0, false, false, 0, true).to_bytes();
    table_data.extend_from_slice(&sprite1);

    for (i, value) in table_data.iter().enumerate() {
        output[i + 6200] = *value;
    }

    let mut rand = nanorand::WyRand::new();

    for i in 0..200 {
        output[7000 + i] = rand.generate();
    }

    output
}

fn altas() -> [u8; 4000] {
    let mut output = [0; 4000];

    let bytes = vec![
        17, 17, 17, 17, 17, 0, 0, 17, 17, 0, 0, 17, 17, 0, 0, 17, 17, 0, 0, 17, 17, 0, 0, 17, 17,
        0, 0, 17, 17, 0, 0, 17, 17, 17, 17, 17,
    ];

    for (i, value) in bytes.iter().enumerate() {
        output[i] = *value;
    }

    output
}

fn controller() -> Vec<[u8; 88]> {
    vec![
        [
            16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16,
            0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0,
            16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16,
            0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0, 16, 0,
        ];
        9
    ]
}
