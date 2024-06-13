use log::debug;
use sdl2::rect::Point;
use sdl2::sys::SDL_Renderer;
use sdl2::{rect::Rect, render::Canvas, video::Window, Sdl};

use crate::components::VideoMemory;
use crate::constants;

pub struct Display {
    pub canvas: Canvas<Window>,
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

    // pub fn clear(&mut self) {
    //     self.canvas.clear();
    // }

    // pub fn present(&mut self) {
    //     self.canvas.present();
    // }

    pub fn refresh<'a>(&mut self, vram: &VideoMemory) {
        let mut draw_pixels: Vec<Rect> = Vec::new();

        // ONLY REDRAW IF SOMETHING MOVES BETWEEN FRAMES
        // save the XOR results - only draw the pixels turned on by the XOR
        // OR do another XOR w/the current and previous vram states

        // What's happening is...
        //  - After each DRW call, the other sprites disappear and only the one
        //    being drawn appears
        //  - When the screen clears, it's only drawing the thing that changed,
        //    i.e. the sprite from the most recent DRW call
        //  - BUT the static numbers at the top don't change
        //  - **This has to be a consequence of XORing the display**

        //  - the first time the ball sprite was drawn, it didn't appear on the screen
        //  - the next frame that drew the ball (in a new position) it did appear
        //  - seems like the XORing turns a pixel off every other frame?
        //  - yes, it does, to erase the previously drawn sprite (e.g. when the ball moves)
        //  - the problem is when the sprite hasn't actually moved

        //  - comparing prev/current VRAM states doesn't work because XOR keeps flipping the value back and forth, so it ends up working the same as w/o the previous VRAM check
        //  - we need to know if a sprite has moved or not, and not try to XOR it if it hasn't
        //  - but we also need to know if it's a genuine collision so we can XOR it
        //  - we can't do this without tracking individual sprites and where they've been drawn
        //  - if a sprite's current drawing location and its last are the same, skip XORing it

        // self.canvas.set_draw_color(constants::FOREGROUND_COLOR);
        for y in 0..constants::SCREEN_HEIGHT {
            for x in 0..constants::SCREEN_WIDTH {
                if *vram.read(x, y) == 1 {
                    draw_pixels.push(Rect::new(
                        (x * constants::VIDEO_SCALE) as i32,
                        (y * constants::VIDEO_SCALE) as i32,
                        constants::VIDEO_SCALE as u32,
                        constants::VIDEO_SCALE as u32,
                    ))
                }
            }
        }

        self.canvas.set_draw_color(constants::BACKGROUND_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(constants::FOREGROUND_COLOR);
        self.canvas
            .fill_rects(&draw_pixels)
            .expect("Failed to draw rectangles");

        self.canvas.present();
    }
}
