use crate::Rom;

struct Registers {
    /// General registers represented as v0-vf in technical docs
    general_registers: [u8; 16],
    /// 16 bit index register used for to store memory addresses
    index: u16,
    /// delay and sound registers, when non-zero they decrement at 60hz
    delay_timer: u8,
    sound_timer: u8,
}

struct Stack {
    /// stack which allows for subroutines, each point to memory addresses
    stack: [u16; 16],
    /// stack pointer which points to location in the stack
    /// https://austinmorlan.com/posts/chip8_emulator/#8-bit-stack-pointer
    stack_pointer: u8,
}

pub struct CPU {
    /// 4kb of internal memory
    memory: [u8; 4096],
    /// program counter, points to current memory location, should be >= 0x000
    program_counter: u16,
    /// Chip registers
    registers: Registers,
    /// stack for subroutines
    stack: Stack,
}

impl CPU {

    /// constructor
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            program_counter: 0x200,
            registers: Registers {
                general_registers: [0; 16],
                index: 0,
                delay_timer: 0,
                sound_timer: 0,
            },
            stack: Stack {
                stack: [0; 16],
                stack_pointer: 0,
            },
        }
    }

    pub fn load_rom(&self, rom: Rom) {

        let mem = &rom.memory[..rom.size];

        print!("{}", mem.len())


    }

}