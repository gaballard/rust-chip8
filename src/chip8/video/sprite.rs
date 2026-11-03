///
/// Sprite Key
///
/// (Vx, Vy, sprite mem start, sprite mem end)
///
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpriteKey(pub u8, pub u8, pub u16, pub u16);

///
/// Sprite
///
/// CHIP-8 sprites are always eight pixels wide and between one to fifteen pixels high.
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Sprite {
    pub key: SpriteKey,
    x: usize,
    y: usize,
    sprite_data: Vec<u8>,
    sprite_len: u8,
}

impl Sprite {
    #[inline]
    pub fn new(key: SpriteKey, sprite_data: Vec<u8>, sprite_len: u8) -> Self {
        Sprite {
            key,
            x: 0,
            y: 0,
            sprite_data,
            sprite_len,
        }
    }

    #[inline]
    pub fn get_sprite_len(&self) -> u8 {
        self.sprite_len
    }

    #[inline]
    pub fn get_sprite_data(&self) -> &Vec<u8> {
        &self.sprite_data
    }

    #[inline]
    pub fn get_x(&self) -> usize {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> usize {
        self.y
    }

    #[inline]
    pub fn set_coords(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
