pub struct Boat {
    acceleration_rate: u32,
    speed_in_ms: u32,
}

impl Boat {
    pub fn build(acceleration_rate: u32) -> Self {
        Boat {
            acceleration_rate,
            speed_in_ms: 0,
        }
    }

    pub fn charge(&mut self, time_in_ms: u32) {
        self.speed_in_ms = self.acceleration_rate * time_in_ms;
    }

    pub fn distance_covered(&self, time_in_ms: u32) -> u32 {
        self.speed_in_ms * time_in_ms
    }
}
