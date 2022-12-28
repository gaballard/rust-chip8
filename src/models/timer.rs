///
/// Timer
///
pub struct Timer {
    value: u8,
    tick: f32,
    clock_speed: usize,
}

impl Timer {
    pub fn new(value: u8, tick: f32, clock_speed: usize) -> Self {
        Self {
            value,
            tick,
            clock_speed,
        }
    }

    pub fn get_value(&self) -> &u8 {
        &self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn clear(&mut self) {
        self.value = 0;
        self.tick = 0.0;
    }

    pub fn update(&mut self, delta_time: &f32) {
        if self.value > 0 {
            self.tick -= delta_time;

            if self.tick <= 0.0 {
                self.value = self.value.checked_sub(1).unwrap_or(0);
                self.tick = 1.0 / self.clock_speed as f32;
            }
        }
    }
}
