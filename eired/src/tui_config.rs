use std::sync::Arc;

use crate::{ConfigBuilder, Point, RuntimeConfig, terminal};

pub struct TuiConfig {
    raw_mode: bool,
    alternate_screen: bool,
    fps: u16,
    base_pos: Point,
    size: Arc<dyn Fn() -> (u16, u16)>,
}

impl TuiConfig {
    pub(crate) fn size(&self) -> Arc<dyn Fn() -> (u16, u16)> {
        self.size.clone()
    }
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            raw_mode: true,
            alternate_screen: true,
            fps: 30,
            base_pos: (0, 0).into(),
            size: Arc::new(terminal::get_size),
        }
    }
}

impl From<TuiConfig> for RuntimeConfig {
    fn from(value: TuiConfig) -> Self {
        ConfigBuilder::default()
            .raw_mode(value.raw_mode)
            .alternate_screen(value.alternate_screen)
            .fps(value.fps)
            .base_pos(value.base_pos)
            .build()
    }
}
