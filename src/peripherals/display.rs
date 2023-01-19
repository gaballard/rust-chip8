use crate::constants;
use crate::utils::read_bit_from_byte;
use crate::{components::VideoMemory, models::Sprite};

use eyre::Result;
use sdl2::rect::Point;
// use log::debug;
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
            refresh_display: false,
        }
    }

    pub fn set_refresh_flag(&mut self, val: bool) {
        self.refresh_display = val;
    }

    pub fn draw_sprite<'a>(&mut self, vram: &VideoMemory, sprite: &Sprite<'a>) {
        // Get VRAM
        // Get sprite data
        // Do an XOR like usual
        // For all pixels that are removed...
        //  Get the corresponding Points from the points array
        //  Set color to BG and draw Points
        // For all pixels that were added...
        //  Get the corresponding Points from the points array
        //  Set color to FG and draw Points

        self.canvas.set_draw_color(constants::BACKGROUND_COLOR);
        let points: [Point; constants::MAX_SPRITE_HEIGHT * constants::MAX_SPRITE_WIDTH] =
            sprite.points.get(sprite.prev_position);
        let _ = self.canvas.draw_points(points.as_slice());

        self.canvas.set_draw_color(constants::FOREGROUND_COLOR);
        let points: [Point; constants::MAX_SPRITE_HEIGHT * constants::MAX_SPRITE_WIDTH] =
            sprite.points(*sprite.position);
        let _ = self.canvas.draw_points(points.as_slice());

        self.canvas.present();
    }

    pub async fn refresh<'a>(&mut self, vram: &VideoMemory<'a>) -> Result<()> {
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

        // Draw to screen
        self.canvas.present();

        Ok(())
    }
}
