use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
const SCALE_FACTOR: u32 = 20;


pub struct  VideoWindow {
    canvas: Canvas<Window>,
}

impl VideoWindow {

    pub fn new(sdl_context: &sdl2::Sdl) -> VideoWindow {
        let video = sdl_context.video().unwrap();
        let window = video.window("Test", SCREEN_WIDTH as u32  * SCALE_FACTOR, SCREEN_HEIGHT as u32 * SCALE_FACTOR)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        VideoWindow {
            canvas: canvas,
        }
    }

    pub fn draw(&mut self, pixels: &[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(VideoWindow::color(col));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }

    fn color(value: u8) -> pixels::Color {
        if value == 0 {
            pixels::Color::RGB(0, 0, 0)
        } else {
            pixels::Color::RGB(0, 250, 0)
        }
    }
    

}