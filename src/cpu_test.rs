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
fn test_load() {

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
    let mut chip = init_cpu();
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
    chip.registers.general_registers[2] = 0x01;

}

