use super::*;

#[test]
fn test_constructor() {
    let chip = CPU::new();
    assert_eq!(chip.program_counter, 0x200);
    assert_eq!(chip.stack.stack_pointer, 0);
    assert_eq!(chip.stack.addresses, [0; 16]);
    assert_eq!(chip.memory[0x50..0x55], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
}

#[test]
fn test_load_rom() {

    let mut chip = CPU::new();
    let mut rom_memory = [0; 3584];
    let rom_size = 3;

    rom_memory[0] = 1;
    rom_memory[1] = 2;
    rom_memory[2] = 3;

    chip.load_rom(Rom {
        memory: rom_memory,
        size: rom_size,
    });

    assert_eq!(chip.memory[0x200], 1);
    assert_eq!(chip.memory[0x201], 2);
    assert_eq!(chip.memory[0x202], 3);
    assert_eq!(chip.memory[0x203], 0)

}

#[test]
fn test_clear() {
    let mut chip = CPU::new();
    chip.video_buffer = [[128; SCREEN_WIDTH]; SCREEN_HEIGHT];
    chip.run_operation(0x00e0);

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            assert_eq!(chip.video_buffer[y][x], 0);
        }
    }

    assert_eq!(chip.program_counter, 0x202)

}

#[test]
fn test_return() {
    let mut chip = CPU::new();
    chip.stack.stack_pointer = 4;
    chip.stack.addresses[3] = 0xfefa;
    chip.run_operation(0x00ee);
    assert_eq!(chip.stack.stack_pointer, 3);
    assert_eq!(chip.program_counter, 0xfefa);
}

#[test]
fn test_jump() {
    let mut chip = CPU::new();
    chip.run_operation(0x1524);
    assert_eq!(chip.program_counter, 0x0524);
}

#[test]
fn test_call() {
    let mut chip = CPU::new();
    chip.run_operation(0x2121);
    assert_eq!(chip.stack.stack_pointer, 1);
    assert_eq!(chip.stack.addresses[0], 0x202);
}

#[test]
fn test_skip_is_equal() {
    let mut chip = CPU::new();
    chip.registers.general_registers[2] = 0x11;
    chip.run_operation(0x3211);
    assert_eq!(chip.program_counter, 0x204);
    chip.run_operation(0x3212);
    assert_eq!(chip.program_counter, 0x206);
}

#[test]
fn test_skip_is_not_equal() {
    let mut chip = CPU::new();
    chip.registers.general_registers[2] = 0x11;
    chip.run_operation(0x4211);
    assert_eq!(chip.program_counter, 0x202);
    chip.registers.general_registers[2] = 0x12;
    chip.run_operation(0x4211);
    assert_eq!(chip.program_counter, 0x206);
}

#[test]
fn test_compare_registers() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0] = 0x21;
    chip.registers.general_registers[1] = 0x23;
    chip.registers.general_registers[2] = 0x21;

    chip.run_operation(0x5200);
    assert_eq!(chip.program_counter, 0x204);

    chip.run_operation(0x5210);
    assert_eq!(chip.program_counter, 0x206);
}

#[test]
fn test_load() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0] = 0x21;
    chip.run_operation(0x6032);
    assert_eq!(chip.registers.general_registers[0], 0x0032);
}

#[test]
fn test_add_to_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[3] = 0x55;
    chip.run_operation(0x7322);
    assert_eq!(chip.registers.general_registers[0x0003], 0x77);
    chip.run_operation(0x73aa);
    assert_eq!(chip.registers.general_registers[0x0003], 0x21);
}
#[test]
fn test_load_from_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[5] = 0x61;
    chip.run_operation(0x8590);
    assert_eq!(chip.registers.general_registers[5], chip.registers.general_registers[9]);
}

#[test]
fn test_set_or() {
    let mut chip = CPU::new();
    chip.registers.general_registers[3] = 0x34;
    chip.registers.general_registers[6] = 0x7f;
    chip.run_operation(0x8361);
    assert_eq!(chip.registers.general_registers[3], 0x34 | 0x7f)
}

#[test]
fn test_set_and() {
    let mut chip = CPU::new();
    chip.registers.general_registers[3] = 0x23;
    chip.registers.general_registers[6] = 0x4f;
    chip.run_operation(0x8362);
    assert_eq!(chip.registers.general_registers[3], 0x23 & 0x4f)
}

#[test]
fn test_set_xor() {
    let mut chip = CPU::new();
    chip.registers.general_registers[3] = 0x96;
    chip.registers.general_registers[6] = 0xfe;
    chip.run_operation(0x8363);
    assert_eq!(chip.registers.general_registers[3], 0x96 ^ 0xfe)
}

#[test]
fn test_add_from_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xa] = 0x22;
    chip.registers.general_registers[0xc] = 0x55;
    chip.registers.general_registers[0x3] = 0xaa;
    chip.run_operation(0x8ac4);
    assert_eq!(chip.registers.general_registers[0xa], 0x22 + 0x55);
    assert_eq!(chip.registers.general_registers[0xf], 0);
    chip.run_operation(0x8a34);
    assert_eq!(chip.registers.general_registers[0xa], 0x21);
    assert_eq!(chip.registers.general_registers[0xf], 1);
}

#[test]
fn test_subtract_from_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xa] = 0x22;
    chip.registers.general_registers[0xc] = 0x55;
    chip.registers.general_registers[0x3] = 0xaa;
    chip.run_operation(0x8ac5);
    assert_eq!(chip.registers.general_registers[0xa], 0xCD);
    assert_eq!(chip.registers.general_registers[0xf], 0);
    chip.run_operation(0x83c5);
    assert_eq!(chip.registers.general_registers[0x3], 0x55);
    assert_eq!(chip.registers.general_registers[0xf], 1);    
}

#[test]
fn test_shift_right() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xe] = 0xa2;
    chip.run_operation(0x8ee6);
    assert_eq!(chip.registers.general_registers[0xe], 0xa2 >> 1);
    assert_eq!(chip.registers.general_registers[0xf], 0);
    chip.registers.general_registers[0xe] = 0xa3;
    chip.run_operation(0x8ee6);
    assert_eq!(chip.registers.general_registers[0xe], 0xa3 >> 1);
    assert_eq!(chip.registers.general_registers[0xf], 1);
}

#[test]
fn test_subtract_no_borrow_from_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xa] = 0x22;
    chip.registers.general_registers[0xc] = 0x55;
    chip.registers.general_registers[0x3] = 0xaa;
    chip.run_operation(0x8ac7);
    assert_eq!(chip.registers.general_registers[0xa], 0x55 - 0x22);
    assert_eq!(chip.registers.general_registers[0xf], 1);
    chip.run_operation(0x83c7);
    assert_eq!(chip.registers.general_registers[0x3], 0xab);
    assert_eq!(chip.registers.general_registers[0xf], 0);    
}

#[test]
fn test_shift_left() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xe] = 0x42;
    chip.run_operation(0x8eee);
    assert_eq!(chip.registers.general_registers[0xe], 0x42 << 1);
    assert_eq!(chip.registers.general_registers[0xf], 0);
    chip.registers.general_registers[0xe] = 0xf1;
    chip.run_operation(0x8eee);
    assert_eq!(chip.registers.general_registers[0xe], 0xf1 << 1);
    assert_eq!(chip.registers.general_registers[0xf], 1);
}

#[test]
fn test_skip_is_not_equal_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0] = 0x21;
    chip.registers.general_registers[1] = 0x23;
    chip.registers.general_registers[2] = 0x21;

    chip.run_operation(0x9010);
    assert_eq!(chip.program_counter, 0x204);

    chip.run_operation(0x9020);
    assert_eq!(chip.program_counter, 0x206);
}

#[test]
fn test_load_index() {
    let mut chip = CPU::new();
    assert_eq!(chip.registers.index, 0);
    chip.run_operation(0xa123);
    assert_eq!(chip.registers.index, 0x123);
    chip.run_operation(0xa125);
    assert_eq!(chip.registers.index, 0x125);
}

#[test]
fn test_jump_plus_vo() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0] = 0x32;
    assert_eq!(chip.program_counter, 0x200);
    chip.run_operation(0xb123);
    assert_eq!(chip.program_counter, 0x32 + 0x123);
}

#[test]
fn test_display() {
    let mut chip = CPU::new();
    chip.registers.index = 0x20;
    chip.memory[0x20] = 0b11111111;
    chip.memory[0x21] = 0b00000000;
    chip.video_buffer[21][20] = 1;
    chip.video_buffer[21][21] = 0;
    chip.video_buffer[22][20] = 1;
    chip.video_buffer[22][21] = 0;
    chip.registers.general_registers[0] = 20;
    chip.registers.general_registers[1] = 21;
    chip.run_operation(0xd012);

    assert_eq!(chip.video_buffer[21][20], 0);
    assert_eq!(chip.video_buffer[21][21], 1);
    assert_eq!(chip.video_buffer[22][20], 1);
    assert_eq!(chip.video_buffer[22][21], 0);
    assert_eq!(chip.registers.general_registers[0x0f], 1);
    assert!(chip.video_changed);
    assert_eq!(chip.program_counter, 0x202);
}

#[test]
fn test_skip_if_key() {
    let mut chip = CPU::new();
    chip.registers.keypad[2] = true;
    chip.registers.general_registers[3] = 2;
    chip.registers.general_registers[4] = 3;
    chip.run_operation(0xE39E);
    assert_eq!(chip.program_counter, 0x204);
    chip.run_operation(0xE49E);
    assert_eq!(chip.program_counter, 0x206);
}

#[test]
fn test_skip_if_not_key() {
    let mut chip = CPU::new();
    chip.registers.keypad[2] = true;
    chip.registers.general_registers[3] = 2;
    chip.registers.general_registers[4] = 3;
    chip.run_operation(0xE3A1);
    assert_eq!(chip.program_counter, 0x202);
    chip.run_operation(0xE4A1);
    assert_eq!(chip.program_counter, 0x206);
}

#[test]
fn test_load_register_from_delay() {
    let mut chip = CPU::new();
    chip.registers.delay_timer = 0x43;
    chip.run_operation(0xF507);
    assert_eq!(chip.registers.general_registers[5], 0x43);
}

#[test]
fn test_wait_key_press() {
    let mut chip = CPU::new();
    assert_eq!(chip.program_counter, 0x200);
    chip.run_operation(0xF50A);
    assert_eq!(chip.program_counter, 0x200);
    chip.run_operation(0xF50A);
    assert_eq!(chip.program_counter, 0x200);
    chip.registers.keypad[3] = true;
    chip.run_operation(0xF50A);
    assert_eq!(chip.program_counter, 0x202);
    assert_eq!(chip.registers.general_registers[5], 3)
}

#[test]
fn test_load_delay_from_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[5] = 0x43;
    chip.run_operation(0xF515);
    assert_eq!(chip.registers.delay_timer, 0x43);
}

#[test]
fn test_load_sound_from_register() {
    let mut chip = CPU::new();
    chip.registers.general_registers[5] = 0x43;
    chip.run_operation(0xF518);
    assert_eq!(chip.registers.sound_timer, 0x43);
}

#[test]
fn test_add_index() {
    let mut chip = CPU::new();
    chip.registers.index = 0x232;
    chip.registers.general_registers[7] = 0x21;
    chip.run_operation(0xF71E);
    assert_eq!(chip.registers.index, 0x232 + 0x21);
}

#[test]
fn test_index_sprite() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xA] = 0x7;
    chip.run_operation(0xFA29);
    assert_eq!(chip.registers.index, 0x73);
}

#[test]
fn test_store_bcd() {
    let mut chip = CPU::new();
    chip.registers.general_registers[0xB] = 0xae;
    chip.run_operation(0xFB33);
    assert_eq!(chip.memory[0x0], 1);
    assert_eq!(chip.memory[0x1], 7);
    assert_eq!(chip.memory[0x2], 4);
}

#[test]
fn test_store_registers() {
    let mut chip = CPU::new();
    let test_registers: [u8; 16] = [122,100,22,76,0,5,21,13,90,32,73,88,23,2,131,1];
    chip.registers.general_registers = test_registers;
    chip.run_operation(0xFF55);

    for i in 0..5 {
        assert_eq!(chip.registers.general_registers[i], chip.memory[i])
    }
}

#[test]
fn test_load_registers_from_index() {
    let mut chip = CPU::new();
    let test_registers: [u8; 16] = [122,100,22,76,0,5,21,13,90,32,73,88,23,2,131,1];
    for i in 0..test_registers.len() {
        chip.memory[i] = test_registers[i];
    }
    chip.run_operation(0xFF65);

    for i in 0..5 {
        assert_eq!(chip.registers.general_registers[i], chip.memory[i])
    }
}