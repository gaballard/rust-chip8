use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl};

use crate::components::VideoMemory;
use crate::constants;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context
            .video()
            .expect("SDL2 video subsystem failed to initialize in Gpu::new");
        let window = video_subsystem
            .window(
                &constants::EMULATOR_NAME,
                (constants::SCREEN_WIDTH * constants::VIDEO_SCALE) as u32,
                (constants::SCREEN_HEIGHT * constants::VIDEO_SCALE) as u32,
            )
            .position_centered()
            .build()
            .expect("SDL2 failed to create window in Gpu::new");
        let canvas = window
            .into_canvas()
            .build()
            .expect("SDL2 failed to initialize window canvas in Gpu::new");

        Display { canvas }
    }

    pub fn refresh<'a>(&mut self, vram: &VideoMemory) {
        self.canvas.set_draw_color(constants::BACKGROUND_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(constants::FOREGROUND_COLOR);
        for y in 0..constants::SCREEN_HEIGHT {
            for x in 0..constants::SCREEN_WIDTH {
                if *vram.read(x, y) == 1 {
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * constants::VIDEO_SCALE) as i32,
                            (y * constants::VIDEO_SCALE) as i32,
                            constants::VIDEO_SCALE as u32,
                            constants::VIDEO_SCALE as u32,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
