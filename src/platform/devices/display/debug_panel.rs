use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

use crate::{chip8::Cpu, constants, platform::Platform};

///
/// Debug Panel GUI
///
pub struct DebugPanel<'a> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    font: &'a Font<'a, 'static>,
    pub padding: u32,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl<'a> DebugPanel<'a> {
    pub fn new(
        platform: &'a Platform,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        padding: u32,
        color_palette: usize,
        video_scale: u32,
        font: &'a Font<'a, 'static>,
    ) -> Self {
        let scaled_width = width * video_scale;
        let scaled_height = height * video_scale;

        // let font = platform
        //     .get_ttf_context()
        //     .load_font(constants::FONT_PATH, constants::FONT_SIZE)
        //     .expect("Font does not exist");

        let video_subsystem = platform
            .get_sdl_context()
            .video()
            .expect("SDL2 video subsystem failed to initialize in Gpu::new");

        let debug_window = video_subsystem
            .window("Debug!", scaled_width + padding, scaled_height)
            .position(x, y)
            .hidden()
            .build()
            .expect("SDL2 failed to create debug window in Gpu::new");

        let canvas = debug_window
            .into_canvas()
            .build()
            .expect("SDL2 failed to initialize debug window canvas in Gpu::new");

        let texture_creator = canvas.texture_creator();

        let colors = constants::PALETTE_COLORS[color_palette];
        let foreground_color = colors.0;
        let background_color = colors.1;

        DebugPanel {
            canvas,
            texture_creator,
            font,
            padding,
            foreground_color,
            background_color,
        }
    }

    pub fn reset(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }

    pub fn set_color_palette(&mut self, color_palette: usize) {
        let colors = constants::PALETTE_COLORS[color_palette];
        self.foreground_color = colors.0;
        self.background_color = colors.1;
    }

    pub fn set_visibility(&mut self, is_visible: bool) {
        if is_visible {
            self.canvas.window_mut().show();
        } else {
            self.canvas.window_mut().hide();
        }
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize) {
        let surface = self
            .font
            .render(text)
            .solid(self.foreground_color)
            .expect("Error drawing text");

        let texture = self
            .texture_creator
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

    pub fn update(&mut self, cpu: &Cpu) {
        self.canvas.set_draw_color(self.foreground_color);

        let width = self.canvas.window().size().0;

        let anchor_x = 0;
        let anchor_y = 0;

        let col = 0;
        let mut row = 0;

        // PC
        self.draw_text(
            &format!("PC: 0x{:X}", cpu.get_pc().address),
            (anchor_x as u32 + self.padding * 2 + (width / 2) * col) as usize,
            ((anchor_y + self.padding as i32 + self.font.height() * row) as f32 * 1.5) as usize,
        );

        // I register
        row += 1;
        self.draw_text(
            &format!("I:  0x{:X}", cpu.i),
            (anchor_x as u32 + self.padding * 2 + (width / 2) * col) as usize,
            ((anchor_y + self.padding as i32 + self.font.height() * row) as f32 * 1.5) as usize,
        );

        // V registers
        for r in 0..16 {
            row += 1;
            self.draw_text(
                &format!("V{:X}: {}", r, cpu.v.read(r)).to_string(),
                (anchor_x as u32 + self.padding * 2 + (width / 2) * col) as usize,
                ((anchor_y + self.padding as i32 + self.font.height() * row) as f32 * 1.5) as usize,
            );
        }

        self.canvas.present();
    }
}
