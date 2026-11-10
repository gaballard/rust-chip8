mod debug_panel;

use std::ops::Mul;

use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::chip8::VideoMemory;
use crate::constants;
use crate::platform::Platform;
use crate::platform::devices::display::debug_panel::DebugPanel;

///
/// Display
///
pub struct Display<'a> {
    canvas: Canvas<Window>,
    pub video_scale: u32,
    pub foreground_color: Color,
    pub background_color: Color,
    pub fading_pixels: Vec<(usize, usize, u8)>,
    pub debug_panel: DebugPanel<'a>,
}

impl<'a> Display<'a> {
    pub fn new(
        platform: &'a Platform,
        hires_mode: bool,
        font: &'a Font<'a, 'static>,
        color_palette: usize,
    ) -> Self {
        let video_scale = constants::VIDEO_SCALE;
        let display_width = if hires_mode {
            constants::HIRES_SCREEN_WIDTH as u32
        } else {
            constants::LORES_SCREEN_WIDTH as u32
        };

        let display_height = if hires_mode {
            constants::HIRES_SCREEN_HEIGHT as u32
        } else {
            constants::LORES_SCREEN_HEIGHT as u32
        };

        let video_subsystem = platform
            .get_sdl_context()
            .video()
            .expect("SDL2 video subsystem failed to initialize in Gpu::new");

        let window = video_subsystem
            .window(
                &constants::EMULATOR_NAME,
                display_width * video_scale,
                display_height * video_scale,
            )
            .position_centered()
            .build()
            .expect("SDL2 failed to create window in Gpu::new");

        let canvas = window
            .into_canvas()
            .build()
            .expect("SDL2 failed to initialize window canvas in Gpu::new");

        let debug_panel = DebugPanel::new(
            // &video_subsystem,
            platform,
            canvas.window().position().0 + (display_width * video_scale) as i32,
            canvas.window().position().1,
            16,
            display_height,
            6,
            color_palette,
            video_scale,
            font,
        );

        let colors = constants::PALETTE_COLORS[color_palette];
        let foreground_color = colors.0;
        let background_color = colors.1;

        Display {
            canvas,
            video_scale,
            background_color,
            foreground_color,
            fading_pixels: Vec::new(),
            debug_panel,
        }
    }

    pub fn reset(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }

    pub fn set_color_palette(&mut self, color_palette: usize) {
        let (foreground_color, background_color) = constants::PALETTE_COLORS[color_palette];
        self.foreground_color = foreground_color;
        self.background_color = background_color;
    }

    pub fn get_hires_mode(&self) -> bool {
        self.canvas.window().size().0 == constants::HIRES_SCREEN_WIDTH as u32
            && self.canvas.window().size().1 == constants::HIRES_SCREEN_HEIGHT as u32
    }

    pub fn set_hires_mode(&mut self, hires_mode: bool) {
        self.canvas
            .window_mut()
            .set_size(
                if hires_mode {
                    constants::HIRES_SCREEN_WIDTH as u32
                } else {
                    constants::LORES_SCREEN_WIDTH as u32
                } * self.video_scale,
                if hires_mode {
                    constants::HIRES_SCREEN_HEIGHT as u32
                } else {
                    constants::LORES_SCREEN_HEIGHT as u32
                } * self.video_scale,
            )
            .expect("Unable to resize window!");
    }

    pub fn update(&mut self, vram: &mut VideoMemory, crt_fade: bool, crt_fade_rate: u8) {
        if crt_fade {
            if self.fading_pixels.len() > 0 {
                vram.vram_changed = true;
            }
        } else {
            self.fading_pixels.clear();
        }

        if vram.vram_changed {
            self.draw(vram);

            if crt_fade {
                self.fade_pixels(crt_fade_rate);
            }

            self.canvas.present();
        }
    }

    fn draw(&mut self, vram: &VideoMemory) {
        for y in 0..vram.get_screen_height() {
            for x in 0..vram.get_screen_width() {
                self.canvas.set_draw_color(if *vram.read(x, y) == 1 {
                    self.fading_pixels.push((x, y, 100));
                    self.foreground_color
                } else {
                    self.background_color
                });

                self.canvas
                    .fill_rect(Rect::new(
                        (x * self.video_scale as usize) as i32,
                        (y * self.video_scale as usize) as i32,
                        self.video_scale as u32,
                        self.video_scale as u32,
                    ))
                    .expect(format!("Failed to draw pixel at {},{}", x, y).as_str());
            }
        }
    }

    fn fade_pixels(&mut self, crt_fade_rate: u8) -> usize {
        let mut to_retain: Vec<bool> = Vec::new();

        for i in 0..self.fading_pixels.len() {
            let mut pixel = self.fading_pixels[i];

            let mut draw_color = self.foreground_color;
            let fade_mult = (pixel.2 as f32) / 100 as f32;
            draw_color.r = (draw_color.r as f32).mul(fade_mult) as u8;
            draw_color.g = (draw_color.g as f32).mul(fade_mult) as u8;
            draw_color.b = (draw_color.b as f32).mul(fade_mult) as u8;

            self.canvas.set_draw_color(
                if draw_color.r < 40 && draw_color.g < 40 && draw_color.b < 40 {
                    self.background_color
                } else {
                    draw_color
                },
            );

            self.canvas
                .fill_rect(Rect::new(
                    (pixel.0 * self.video_scale as usize) as i32,
                    (pixel.1 * self.video_scale as usize) as i32,
                    self.video_scale as u32,
                    self.video_scale as u32,
                ))
                .expect(format!("Failed to fade pixel at {},{}", pixel.0, pixel.1).as_str());

            pixel.2 = pixel.2.checked_sub(crt_fade_rate).unwrap_or(0);

            if pixel.2 == 0 {
                to_retain.push(false);
            } else {
                to_retain.push(true);
                self.fading_pixels[i] = pixel;
            }
        }

        if to_retain.len() > 0 {
            let mut iter = to_retain.iter();
            self.fading_pixels.retain(|_| *iter.next().unwrap());
        }

        self.fading_pixels.len()
    }
}
