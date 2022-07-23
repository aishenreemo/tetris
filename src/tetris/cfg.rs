use std::time::Duration;

pub struct Settings {
    pub window_size: (u32, u32),
    pub speed: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_size: (500, 700),
            speed: Duration::from_millis(200),
        }
    }
}
