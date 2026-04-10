use std::io::{self, Stdout, Write};
use std::sync::Arc;

use crossbeam::channel::Sender;

use crate::config::{ConfigBuilder, RuntimeConfig};
use crate::error::{self, Error};
use crate::runtime::{Frame, RenderRuntime, RuntimeTask};
use crate::terminal::{self, Point, TerminalRenderer};

pub struct TuiEngine<W: Write> {
    runtime: RenderRuntime<W, TerminalRenderer<W>>,
    frame: Frame,
    tx: Sender<RuntimeTask>,
}

impl<W: Write + Send + 'static> TuiEngine<W> {
    pub fn run<F: FnOnce(&mut Frame) -> crate::Result<()>>(mut self, process: F) {
        let tx = self.tx;
        let handle = self.runtime.spawn();

        let procs =
            process(&mut self.frame).and_then(|_| tx.send(RuntimeTask::Close).map_err(Error::Send));

        if let Err(e) = procs {
            error::handle_err(e);
        }

        handle.join().ok();
    }

    pub fn run_lazy<F: FnOnce(&mut Frame) -> crate::Result<()> + Send + 'static>(
        mut self,
        process: F,
    ) -> impl Future<Output = ()> {
        let tx = self.tx;
        let handle = self.runtime.spawn();

        async move {
            let procs = process(&mut self.frame)
                .and_then(|_| tx.send(RuntimeTask::Close).map_err(Error::Send));

            if let Err(e) = procs {
                error::handle_err(e);
            }

            handle.join().ok();
        }
    }
}

impl Default for TuiEngine<Stdout> {
    fn default() -> Self {
        let config = TuiConfig::default();
        let size = config.size();
        let config = config.into();
        let renderer = TerminalRenderer::new(io::stdout());
        let (runtime, tx) = RenderRuntime::new(config, renderer);
        let frame = Frame::new(size, tx.clone());

        Self { runtime, frame, tx }
    }
}

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
