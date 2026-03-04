#[derive(Clone, Copy)]
pub struct RuntimeConfig {
    fps: u16,
    pub base_pos: (u16, u16),
    pub alternate_screen: bool,
    pub raw_mode: bool,
}

impl RuntimeConfig {
    pub fn get_fps_tick(&self) -> f64 {
        1.0 / self.fps as f64
    }
}
