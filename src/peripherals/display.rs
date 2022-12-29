use crate::components::VideoMemory;
use crate::constants;

use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Display {
    canvas: Canvas<Window>,
    refresh_display: bool,
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

        Display {
            canvas,
            refresh_display: true,
        }
    }

    pub fn set_refresh_flag(&mut self, val: bool) {
        self.refresh_display = val;
    }

    pub fn refresh(&mut self, vram: &VideoMemory) {
        if self.refresh_display {
            for y in 0..constants::SCREEN_HEIGHT {
                for x in 0..constants::SCREEN_WIDTH {
                    if *vram.read(x, y) == 1 {
                        self.canvas.set_draw_color(constants::FOREGROUND_COLOR);
                    } else {
                        self.canvas.set_draw_color(constants::BACKGROUND_COLOR);
                    }
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
            self.canvas.present();
            self.set_refresh_flag(false);
        }
    }
}
