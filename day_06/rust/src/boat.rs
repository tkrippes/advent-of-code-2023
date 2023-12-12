pub struct Boat {
    acceleration_rate: u64,
    speed_in_ms: u64,
}

impl Boat {
    pub fn build(acceleration_rate: u64) -> Self {
        Boat {
            acceleration_rate,
            speed_in_ms: 0,
        }
    }

    pub fn charge(&mut self, time_in_ms: u64) {
        self.speed_in_ms = self.acceleration_rate * time_in_ms;
    }

    pub fn distance_covered(&self, time_in_ms: u64) -> u64 {
        self.speed_in_ms * time_in_ms
    }
}
