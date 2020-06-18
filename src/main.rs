extern crate rand;
extern crate sdl2;

use std::env;

mod cpu;
mod rom;
mod font;
mod video;
mod input;

use std::thread;
use std::time::Duration;
use cpu::CPU;
use rom::Rom;
use video::VideoWindow;
use input::Input;


const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let sleep_duration = Duration::from_millis(1);

    let mut display = VideoWindow::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut chip = CPU::new();
    let filename = env::args().nth(1);

    let current_rom = match filename {
        Some(filename) => {
            Rom::new(&filename)
        },
        None => {
            panic!("Uh Oh: File not read");
        }
    };

    chip.load_rom(current_rom);

    while let Ok(keypad) = input.poll() {

        let state = chip.tick(keypad);

        if state.video_changed {
            display.draw(state.video_buffer);
        }

        thread::sleep(sleep_duration);

    }
    
    
}
