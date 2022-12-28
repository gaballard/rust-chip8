use crate::components::VideoMemory;
use crate::constants::{EMULATOR_NAME, SCREEN_HEIGHT, SCREEN_WIDTH, VIDEO_SCALE};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

const FG_COLOR: Color = Color::RGB(255, 255, 255);
const BG_COLOR: Color = Color::RGB(0, 0, 0);

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
                &EMULATOR_NAME,
                (SCREEN_WIDTH * VIDEO_SCALE) as u32,
                (SCREEN_HEIGHT * VIDEO_SCALE) as u32,
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
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    if *vram.read(x, y) == 1 {
                        self.canvas.set_draw_color(FG_COLOR);
                    } else {
                        self.canvas.set_draw_color(BG_COLOR);
                    }
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * VIDEO_SCALE) as i32,
                            (y * VIDEO_SCALE) as i32,
                            VIDEO_SCALE as u32,
                            VIDEO_SCALE as u32,
                        ))
                        .unwrap();
                }
            }
            self.canvas.present();
            self.set_refresh_flag(false);
        }
    }
}
