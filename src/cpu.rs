use crate::Rom;
use crate::font::FONT;

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
    pub memory: [u8; 4096],
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

        // load in built in fonts into memory
        let mem = CPU::load_fonts();

        CPU {
            memory: mem,
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

    pub fn load_rom(&mut self, rom: Rom) {

        let mem = &rom.memory[..rom.size];

        for (i, &byte) in mem.iter().enumerate() {

            let memory_address = 0x200 + i;

            if memory_address < self.memory.len() {
                self.memory[memory_address] = byte;
            }
        }

    }

    fn load_fonts() -> [u8; 4096] {

        let mut mem = [0; 4096];
        let font_start_address = 0x50;

        for (i, &byte) in FONT.iter().enumerate() {
            mem[font_start_address + i] = byte;
        }
        
        mem
    }

}