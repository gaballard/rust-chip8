use sdl2::ttf::Sdl2TtfContext;

pub struct Platform {
    sdl_context: sdl2::Sdl,
    ttf_context: Sdl2TtfContext,
}

impl Platform {
    pub fn new() -> Self {
        // Set up SDL context
        let sdl_context = sdl2::init().expect("SDL2 context failed to initialize in main");
        let ttf_context = sdl2::ttf::init().expect("Failed to initialize TTF manager");

        Platform {
            sdl_context,
            ttf_context,
        }
    }

    pub fn get_sdl_context(&self) -> &sdl2::Sdl {
        &self.sdl_context
    }

    pub fn get_ttf_context(&self) -> &Sdl2TtfContext {
        &self.ttf_context
    }
}
