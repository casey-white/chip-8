extern crate rand;

use std::env;

mod cpu;
mod rom;
mod font;

use cpu::CPU;
use rom::Rom;


const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;
const SCREEN_SCALE: usize = 32;


fn main() {

    let mut chip = CPU::new();
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

    for (i, &byte) in chip.memory.iter().enumerate() {
        if byte > 0 {
            println!("{}: {}", i, byte)
        }
    }
    
    
}
