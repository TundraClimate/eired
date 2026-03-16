#![allow(dead_code)]

use std::io::{self, Stdout, Write};

use crossbeam::channel::Sender;

use eired_runtime::RenderRuntime;
use eired_runtime::config::{ConfigBuilder, RuntimeConfig};
use eired_runtime::task::RuntimeTask;
use eired_runtime::terminal::TerminalRenderer;

pub struct TuiEngine<W: Write> {
    runtime: RenderRuntime<W, TerminalRenderer<W>>,
    tx: Sender<RuntimeTask>,
}

impl<W: Write + Send + 'static> TuiEngine<W> {
    pub fn run<F: FnOnce()>(self, process: F) {
        let tx = self.tx;
        let _guard = ClosingGuard { tx: tx.clone() };

        self.runtime.spawn();
        process()
    }

    pub fn run_lazy<F: FnOnce() + Send + 'static>(self, process: F) -> impl Future<Output = ()> {
        let tx = self.tx;
        let guard = ClosingGuard { tx: tx.clone() };

        self.runtime.spawn();

        async move {
            let _guard = guard;

            process()
        }
    }
}

impl Default for TuiEngine<Stdout> {
    fn default() -> Self {
        let config = TuiConfig::default().into();
        let renderer = TerminalRenderer::new(io::stdout());
        let (runtime, tx) = RenderRuntime::new(config, renderer);

        Self { runtime, tx }
    }
}

pub struct TuiConfig {
    raw_mode: bool,
    alternate_screen: bool,
    fps: u16,
    base_pos: (u16, u16),
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            raw_mode: true,
            alternate_screen: true,
            fps: 30,
            base_pos: (0, 0),
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

struct ClosingGuard {
    tx: Sender<RuntimeTask>,
}

impl Drop for ClosingGuard {
    fn drop(&mut self) {
        self.tx.send(RuntimeTask::Close).ok();
    }
}
