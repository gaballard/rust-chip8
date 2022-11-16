// use crate::SCREEN_HEIGHT;
// use crate::SCREEN_WIDTH;

// pub struct Gpu {
//     vram: [[usize; SCREEN_WIDTH]; SCREEN_HEIGHT],
//     vram_changed: bool,
// }

// impl Gpu {
//     pub fn new() -> Gpu {
//         let gpu = Gpu {
//             vram: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
//             vram_changed: false,
//         };

//         gpu
//     }

//     pub fn write_vram_byte(&mut self, x: usize, y: usize, value: usize) {
//         self.vram[x][y] = value;
//     }

//     pub fn read_vram_byte(&self, x: usize, y: usize) -> usize {
//         self.vram[x][y]
//     }

//     pub fn set_vram_changed(&mut self, value: bool) {
//         self.vram_changed = value;
//     }

//     pub fn check_vram_changed(&mut self) {}

//     pub fn clear_display(&mut self) {
//         let mut x = 0;
//         let mut y = 0;
//         for _ in self.vram {
//             for _ in self.vram[x] {
//                 self.vram[x][y] = 0;
//                 y += 1;
//             }
//             x += 1;
//         }
//     }

//     pub fn draw_sprite(&mut self) {}

//     pub fn draw_font(&mut self) {}
// }
