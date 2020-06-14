use std::env;

mod cpu;
mod rom;

use cpu::CPU;
use rom::Rom;


fn main() {

    let chip = CPU::new();
    let filename = env::args().nth(1);

    let current_rom = match filename {
        Some(filename) => {
            Rom::new(&filename)
        },
        None => {
            panic!("Uh Oh");
        }
    };

    chip.load_rom(current_rom);
    
    
}
