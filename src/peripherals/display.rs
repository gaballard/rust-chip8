use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::components::VideoMemory;
use crate::constants;
use crate::platform::Platform;

pub struct Display<'a> {
    pub canvas: Canvas<Window>,
    pub font: Font<'a, 'static>,
    _texture_creator: TextureCreator<WindowContext>,
    pub display_scale_factor: usize,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl<'a> Display<'a> {
    pub fn new(platform: &'a Platform) -> Self {
        let display_scale_factor = constants::VIDEO_SCALE;
        let background_color = constants::BACKGROUND_COLOR;
        let foreground_color = constants::FOREGROUND_COLOR;

        let video_subsystem = platform
            .get_sdl_context()
            .video()
            .expect("SDL2 video subsystem failed to initialize in Gpu::new");

        let window = video_subsystem
            .window(
                &constants::EMULATOR_NAME,
                (constants::SCREEN_WIDTH * 2 * display_scale_factor + 20) as u32,
                (constants::SCREEN_HEIGHT * 2 * display_scale_factor + 20) as u32,
            )
            .position_centered()
            .build()
            .expect("SDL2 failed to create window in Gpu::new");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("SDL2 failed to initialize window canvas in Gpu::new");

        let font = platform
            .get_ttf_context()
            .load_font("./fonts/SperryPC_CGA.ttf", 16)
            .expect("Font does not exist");

        let _texture_creator = canvas.texture_creator();

        // Reset display
        canvas.set_draw_color(background_color);
        canvas.clear();
        canvas.present();

        Display {
            canvas,
            font,
            _texture_creator,
            display_scale_factor,
            background_color,
            foreground_color,
        }
    }

    pub fn _draw_text(&mut self, text: &str, x: usize, y: usize) {
        let surface = self
            .font
            .render(text)
            .solid(self.foreground_color)
            .expect("Error drawing text");

        let texture = self
            ._texture_creator
            .create_texture_from_surface(surface)
            .expect("Failed to create texture from font surface");

        let query = texture.query();
        let text_width = query.width;
        let text_height = query.height;

        let rect = Rect::new(x as i32, y as i32, text_width, text_height);

        self.canvas
            .copy(&texture, None, Some(rect))
            .expect("Error rendering texture");
    }

    pub fn draw(&mut self, vram: &VideoMemory) {
        let mut pixels: Vec<Rect> = Vec::new();

        for y in 0..vram.get_screen_height() {
            for x in 0..vram.get_screen_width() {
                if *vram.read(x as usize, y as usize) == 1 {
                    pixels.push(Rect::new(
                        (x * self.display_scale_factor as usize) as i32,
                        (y * self.display_scale_factor as usize) as i32,
                        self.display_scale_factor as u32,
                        self.display_scale_factor as u32,
                    ))
                }
            }
        }

        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();

        self.canvas.set_draw_color(self.foreground_color);
        self.canvas
            .fill_rects(&pixels)
            .expect("Failed to draw pixels");

        self.canvas.present();
    }
}
