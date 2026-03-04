use std::time::{Duration, Instant};

use crossbeam::channel::{self, Receiver};

#[derive(Default)]
pub struct ConfigBuilder {
    base_pos: (u16, u16),
    alternate_screen: bool,
    raw_mode: bool,
    receiver: Option<Receiver<Instant>>,
}

impl ConfigBuilder {
    pub fn base_pos(mut self, base_pos: (u16, u16)) -> Self {
        self.base_pos = base_pos;

        self
    }

    pub fn alternate_screen(mut self, enter: bool) -> Self {
        self.alternate_screen = enter;

        self
    }

    pub fn raw_mode(mut self, enable: bool) -> Self {
        self.raw_mode = enable;

        self
    }

    pub fn fps(mut self, fps_sec: u16) -> Self {
        self.receiver = Some(channel::tick(Duration::from_secs_f64(1.0 / fps_sec as f64)));

        self
    }

    pub fn ticker(mut self, ticker: Receiver<Instant>) -> Self {
        self.receiver = Some(ticker);

        self
    }

    pub fn no_tick(mut self) -> Self {
        self.receiver = None;

        self
    }

    pub fn build(self) -> RuntimeConfig {
        RuntimeConfig {
            ticker: self.receiver,
            base_pos: self.base_pos,
            alternate_screen: self.alternate_screen,
            raw_mode: self.raw_mode,
        }
    }
}

#[derive(Clone)]
pub struct RuntimeConfig {
    pub(crate) ticker: Option<Receiver<Instant>>,
    pub(crate) base_pos: (u16, u16),
    pub(crate) alternate_screen: bool,
    pub(crate) raw_mode: bool,
}
