use rand;
use rand::Rng;

use crate::Rom;

use crate::font::FONT;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;


const OPCODE_SIZE: u16 = 2;

// Again references https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs#L11
pub struct State<'a> {
    // Lifetime, will refernece the video buffer array from cpu
    pub video_buffer: &'a [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    pub video_changed: bool,
    pub beep: bool,
}

struct Registers {
    /// General registers represented as v0-vf in technical docs
    general_registers: [u8; 16],
    /// 16 bit index register used for to store memory addresses
    index: u16,
    /// delay and sound registers, when non-zero they decrement at 60hz
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
}

struct Stack {
    /// stack which allows for subroutines, each point to memory addresses
    addresses: [u16; 16],
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
    /// video location
    pub video_buffer: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    video_changed: bool,
}


// PCActions enum idea ripped pretty much from:
// https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs
enum PCActions {
    Next,
    Skip,
    StepBack,
    Jump(u16),
}

impl PCActions {
    fn skip_if(condition: bool) -> PCActions {
        if condition { PCActions::Skip } else { PCActions::Next } 
    }
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
                keypad: [false; 16],
            },
            stack: Stack {
                addresses: [0; 16],
                stack_pointer: 0,
            },
            video_buffer: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            video_changed: false,
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

    // Represents an operation occuring and the things that happen around that
    pub fn tick(&mut self, keypad: [bool; 16]) -> State {

        self.registers.keypad = keypad;
        self.video_changed = false;

        if self.registers.delay_timer > 0 {
            self.registers.delay_timer -= 1
        }
        if self.registers.sound_timer > 0 {
            self.registers.sound_timer -= 1
        }
        let operation = self.get_operation();
        self.run_operation(operation);

        State {
            video_buffer: &self.video_buffer,
            video_changed: self.video_changed,
            beep: self.registers.sound_timer > 0,
        }

    }

    // Take the next two codes and combine them into an u16 bit opcode
    fn get_operation(&self) -> u16 {
        (self.memory[self.program_counter as usize] as u16) << 8 | (self.memory[(self.program_counter + 1) as usize] as u16)
    }

    fn run_operation(&mut self, operation: u16) {
        // mask out 4 nibbles
        let nibbles = (
            (operation & 0xF000) >> 12 as u8,
            (operation & 0x0F00) >> 8 as u8,
            (operation & 0x00F0) >> 4 as u8,
            (operation & 0x000F) as u8,
        );

        // mask out last three nibbles for some operations
        let nnn = (operation & 0x0FFF) as u16;
        // mask out 2 nibbles
        let kk = (operation & 0x00FF) as u8;
        // get individual reference to last three nibbles
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        let pc_action = match nibbles { 
            (0x00, 0x00, 0x0e, 0x00) => self.clear(),
            (0x00, 0x00, 0x0e, 0x0e) => self.return_operation(),
            (0x01, _, _, _) => CPU::jump(nnn),
            (0x02, _, _, _) => self.call(nnn),
            (0x03, _, _, _) => self.skip_is_equal(x, kk),
            (0x04, _, _, _) => self.skip_is_not_equal(x, kk),
            (0x05, _, _, 0x00) => self.compare_registers(x, y),
            (0x06, _, _, _) => self.load(x, kk),
            (0x07, _, _, _) => self.add_to_register(x, kk),
            (0x08, _, _, 0x00) => self.load_from_register(x, y),
            (0x08, _, _, 0x01) => self.set_or(x, y),
            (0x08, _, _, 0x02) => self.set_and(x, y),
            (0x08, _, _, 0x03) => self.set_xor(x, y),
            (0x08, _, _, 0x04) => self.add_from_register(x, y),
            (0x08, _, _, 0x05) => self.subtract_from_register(x, y),
            (0x08, _, _, 0x06) => self.shift_right(x),
            (0x08, _, _, 0x07) => self.subtract_no_borrow_from_register(x, y),
            (0x08, _, _, 0x0e) => self.shift_left(x),
            (0x09, _, _, 0x00) => self.skip_is_not_equal_register(x, y),
            (0x0a, _, _, _) => self.load_index(nnn),
            (0x0b, _, _, _) => self.jump_plus_vo(nnn),
            (0x0c, _, _, _) => self.random(x, kk),
            (0x0d, _, _, _) => self.display(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.skip_if_key(x),
            (0x0e, _, 0x0a, 0x01) => self.skip_if_not_key(x),
            (0x0f, _, 0x00, 0x07) => self.load_register_from_delay(x),
            (0x0f, _, 0x00, 0x0a) => self.wait_key_press(x),
            (0x0f, _, 0x01, 0x05) => self.load_delay_from_register(x),
            (0x0f, _, 0x01, 0x08) => self.load_sound_from_register(x),
            (0x0f, _, 0x01, 0x0e) => self.add_index(x),
            (0x0f, _, 0x02, 0x09) => self.index_sprite(x),
            (0x0f, _, 0x03, 0x03) => self.store_bcd(x),
            (0x0f, _, 0x05, 0x05) => self.store_registers(x),
            (0x0f, _, 0x06, 0x05) => self.load_registers_from_index(x),
            _ => PCActions::Next,
        };

        match pc_action {
            PCActions::Next => self.program_counter += OPCODE_SIZE,
            PCActions::Skip => self.program_counter += 2 * OPCODE_SIZE,
            PCActions::Jump(addr) => self.program_counter = addr,
            PCActions::StepBack => (),
        }

    }

    // CLS: Clears video memory
    fn clear(&mut self) -> PCActions {

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.video_buffer[y][x] = 0;
            }
        }

        self.video_changed = true;

        PCActions::Next
    }

    // RET: Acts as a return. Decrements the stack and returns
    // to the location on top of stack.
    fn return_operation(&mut self) -> PCActions {
        self.stack.stack_pointer -= 1;
        PCActions::Jump(self.stack.addresses[self.stack.stack_pointer as usize])
    }

    // JP: Jumps to a certain address (does not alter stack)
    fn jump(location: u16) -> PCActions {
        PCActions::Jump(location)
    }

    // CALL: jumps to address and increments the stack with previous address.
    fn call(&mut self, location: u16) -> PCActions {
        self.stack.addresses[self.stack.stack_pointer as usize] = self.program_counter + OPCODE_SIZE;
        self.stack.stack_pointer += 1;
        PCActions::Jump(location)
    }

    // Skip if data in register is equivalent to input data.
    fn skip_is_equal(&self, register: usize, data: u8) -> PCActions {
        PCActions::skip_if(self.registers.general_registers[register] == data)
    }

    // Skip if data in register is not equivalent to input data.
    fn skip_is_not_equal(&self, register: usize, data: u8) -> PCActions {
        PCActions::skip_if(self.registers.general_registers[register] != data)
    }

    // Skip if the two registers being compared are equivalent,
    fn compare_registers(&self, register_x: usize, register_y: usize) -> PCActions {
        PCActions::skip_if(self.registers.general_registers[register_x] == self.registers.general_registers[register_y])
    }

    // Load data into selected register
    fn load(&mut self, register: usize, data: u8) -> PCActions {
        self.registers.general_registers[register] = data;
        PCActions::Next
    }

    // Adds data to the data in register
    fn add_to_register(&mut self, register: usize, data: u8) -> PCActions {
        self.registers.general_registers[register] = ((self.registers.general_registers[register] as u16) + (data as u16)) as u8;
        PCActions::Next
    }

    // Load data from register y into register x
    fn load_from_register(&mut self, register_x: usize, register_y: usize) -> PCActions {
        self.registers.general_registers[register_x] = self.registers.general_registers[register_y];
        PCActions::Next
    }

    // Load the result of the or operation on register y and register x
    fn set_or(&mut self, register_x: usize, register_y: usize) -> PCActions {
        self.registers.general_registers[register_x] |= self.registers.general_registers[register_y];
        PCActions::Next
    }

    // Load the result of the and operation on register y and register x
    fn set_and(&mut self, register_x: usize, register_y: usize) -> PCActions {
        self.registers.general_registers[register_x] &= self.registers.general_registers[register_y];
        PCActions::Next
    }

    // Load the result of the xor operation on register y and register x
    fn set_xor(&mut self, register_x: usize, register_y: usize) -> PCActions {
        self.registers.general_registers[register_x] ^= self.registers.general_registers[register_y];
        PCActions::Next
    }

    // Load the result of the addition operation on register y and register x
    fn add_from_register(&mut self, register_x: usize, register_y: usize) -> PCActions {
        let x_value = self.registers.general_registers[register_x] as u16;
        let y_value = self.registers.general_registers[register_y] as u16;
        let result = x_value + y_value;
        // Send result to carry register (register vf)
        self.registers.general_registers[0x0F] = if result > 0xFF { 1 } else { 0 };
        self.registers.general_registers[register_x] = result as u8;
        PCActions::Next
    }

    // Subtraction with wrapping between the two registers.
    // If something breaks, CHECK THIS
    fn subtract_from_register(&mut self, register_x: usize, register_y: usize) -> PCActions {
        let x_value = self.registers.general_registers[register_x] as u8;
        let y_value = self.registers.general_registers[register_y] as u8;
        self.registers.general_registers[0x0F] = if x_value > y_value { 1 } else { 0 };
        self.registers.general_registers[register_x] = x_value.wrapping_sub(y_value);
        PCActions::Next
    }

    // Shifts register left and moves value into carry register if odd
    fn shift_right(&mut self, register: usize) -> PCActions {
        self.registers.general_registers[0x0F] = self.registers.general_registers[register] & 1;
        self.registers.general_registers[register] >>= 1;
        PCActions::Next
    }

    // subtracts the x register from the y register
    fn subtract_no_borrow_from_register(&mut self, register_x: usize, register_y: usize) -> PCActions {
        let x_value = self.registers.general_registers[register_x] as u8;
        let y_value = self.registers.general_registers[register_y] as u8;
        self.registers.general_registers[0x0F] = if y_value > x_value { 1 } else { 0 };
        self.registers.general_registers[register_x] = y_value.wrapping_sub(x_value);
        PCActions::Next
    }

    // Shifts register right and moves value into carry register if is 1
    fn shift_left(&mut self, register: usize) -> PCActions {
        self.registers.general_registers[0x0F] = (self.registers.general_registers[register] & 0b10000000) >> 7;
        self.registers.general_registers[register] <<= 1;
        PCActions::Next
    }

    // Skip if registers are not equal
    fn skip_is_not_equal_register(&mut self, register_x: usize, register_y: usize) -> PCActions {
        PCActions::skip_if(self.registers.general_registers[register_x] != self.registers.general_registers[register_y]) 
    }

    // Load data into the index register
    fn load_index(&mut self, data: u16) -> PCActions {
        self.registers.index = data;
        PCActions::Next
    }

    // JP V0, addr
    // The program counter is set to nnn plus the value of V0.
    fn jump_plus_vo(&mut self, data: u16) -> PCActions {
        PCActions::Jump((self.registers.general_registers[0] as u16) + data)
    }

    // Generates a random 8-bit unsigned int, which is ANDed and stored in Vx
    fn random(&mut self, register: usize, data: u8) -> PCActions {
        let mut rng = rand::thread_rng();
        self.registers.general_registers[register] = rng.gen::<u8>() & data;
        PCActions::Next
    }

    // Fairly shamelessly ripped from here:
    // https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs
    // Draws sprite to screen
    fn display(&mut self, register_x: usize, register_y: usize, num_of_bytes: usize) -> PCActions {
        

        // reset collision register
        self.registers.general_registers[0x0F] = 0;

        // n number of bytes stored in I register
        for row in 0..num_of_bytes {
            let y: usize = (self.registers.general_registers[register_y] as usize + row) % SCREEN_HEIGHT;
            let current_byte = self.memory[(self.registers.index as usize + row)];
            // We know 8 columns because a byte is 8
            for col in 0..8 {
                let x: usize = (self.registers.general_registers[register_x] as usize + col) % SCREEN_WIDTH;
                let bit_on = (current_byte >> (7 - col)) & 1;
                // check if the bits overlap
                println!("x : {}, y : {}", x , y);
                self.registers.general_registers[0x0F] |= bit_on & self.video_buffer[y][x];
                // xor bit status with what is in video buffer at that position
                self.video_buffer[y][x] ^= bit_on;
            }
        }

        self.video_changed = true;
        PCActions::Next
    }

    // Skip if selected key (by register) is pressed
    fn skip_if_key(&self, register: usize) -> PCActions {
        PCActions::skip_if(self.registers.keypad[self.registers.general_registers[register] as usize])
    }

    // Skip if selected key (by register) is not pressed
    fn skip_if_not_key(&self, register: usize) -> PCActions {
        PCActions::skip_if(!self.registers.keypad[self.registers.general_registers[register] as usize])
    }

    // Load delay timer value into selected register
    fn load_register_from_delay(&mut self, register: usize) -> PCActions {
        self.registers.general_registers[register] = self.registers.delay_timer;
        PCActions::Next
    }

    // Wait for keypress, if no keypress occurs stepback the PC counter
    fn wait_key_press(&mut self, register: usize) -> PCActions {
        for i in 0..self.registers.keypad.len() {
            if self.registers.keypad[i] {
                self.registers.general_registers[register] = i as u8;
                return PCActions::Next
            }
        }
        PCActions::StepBack
    }

    // Set delay timer to the value in selected register
    fn load_delay_from_register(&mut self, register: usize) -> PCActions {
        self.registers.delay_timer = self.registers.general_registers[register];
        PCActions::Next
    }

    // Set delay timer to the value in selected register
    fn load_sound_from_register(&mut self, register: usize) -> PCActions {
        self.registers.sound_timer = self.registers.general_registers[register];
        PCActions::Next
    }

    // Add to index from the value in the selected register
    fn add_index(&mut self, register: usize) -> PCActions {
        self.registers.index += self.registers.general_registers[register] as u16;
        PCActions::Next
    }

    // Load a sprite to the index register based on value in selected register
    fn index_sprite(&mut self, register: usize) -> PCActions {

        // offset to the location in memory where the font set is saved
        let offset = 0x50;
        let register_value = self.registers.general_registers[register];

        // Each font character is 5 bytes.
        self.registers.index = offset + (5 * register_value) as u16;

        PCActions::Next
    }

    // Store a binary coded decimal in locations Index...Index+2 from selected index
    fn store_bcd(&mut self, register: usize) -> PCActions {

        let register_value = self.registers.general_registers[register];
        let index: usize = self.registers.index as usize;

        self.memory[index] = register_value / 100;
        self.memory[index + 1] = (register_value % 100) / 10;
        self.memory[index + 2] = register_value % 10;

        PCActions::Next

    }

    fn store_registers(&mut self, register: usize) -> PCActions {

        for i in 0..register + 1 {
            self.memory[self.registers.index as usize + i] = self.registers.general_registers[i];
        }

        PCActions::Next

    }

    fn load_registers_from_index(&mut self, register: usize) -> PCActions {

        for i in 0..register + 1 {
            self.registers.general_registers[i] = self.memory[self.registers.index as usize + i]
        }

        PCActions::Next

    }

}

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_test;