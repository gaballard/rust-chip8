use sdl2::rect::Point;

use crate::constants;

const MAX_SPRITE_LENGTH: usize = constants::MAX_SPRITE_HEIGHT * constants::MAX_SPRITE_WIDTH;

///
/// Sprite
///
#[derive(Copy, Clone, Debug)]
pub struct Sprite<'a> {
    data: &'a [u8],
    pub points: [Point; MAX_SPRITE_LENGTH],
    pub position: [usize; 2],
    pub prev_position: [usize; 2],
}

impl<'a> Sprite<'a> {
    pub fn new(data: &'a [u8], position: [usize; 2]) -> Self {
        let mut points = [Point::new(0, 0); MAX_SPRITE_LENGTH];

        let x = position[0];
        let y = position[1];

        let mut sx: usize = 0;
        let mut sy: usize = 0;

        while sy < data.len() / 8 {
            while sx < 8 {
                let mut point = points[sx * sy];
                // points[sx * sy] = Point::new((x + sx) as i32, (y + sy) as i32);
                point.x = (x + sx) as i32;
                point.y = (y + sy) as i32;
                sx += 1;
            }
            sx = 0;
            sy += 1;
        }

        Self {
            data,
            position,
            prev_position: [0; 2],
            points,
        }
    }

    pub fn x(&self) -> &usize {
        &self.position[0]
    }

    pub fn y(&self) -> &usize {
        &self.position[1]
    }

    pub fn position(&self) -> &[usize; 2] {
        &self.position
    }

    pub fn prev_position(&self) -> &[usize; 2] {
        &self.prev_position
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_position(&mut self, position: [usize; 2]) {
        if position != self.position {
            let mut x = position[0];
            if x > constants::SCREEN_WIDTH - 1 {
                x = x % constants::SCREEN_WIDTH;
            }

            let mut y = position[1];
            if y > constants::SCREEN_HEIGHT - 1 {
                y = y % constants::SCREEN_HEIGHT;
            }

            self.prev_position = self.position.clone();
            self.position = [x, y];
        }
    }
}
