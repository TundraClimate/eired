#![allow(dead_code)]

use std::io::{self, Stdout, Write};
use std::mem;

use crossbeam::channel::{SendError, Sender};

use eired_display::{Annot, Annotate, VTerm, View, Window};
use eired_runtime::RenderRuntime;
use eired_runtime::config::{ConfigBuilder, RuntimeConfig};
use eired_runtime::task::RuntimeTask;
use eired_runtime::terminal::TerminalRenderer;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    Send(SendError<RuntimeTask>),
}

fn handle_err(_err: Error) {
    todo!()
}

pub struct TuiEngine<W: Write> {
    runtime: RenderRuntime<W, TerminalRenderer<W>>,
    frame: Frame,
    tx: Sender<RuntimeTask>,
}

impl<W: Write + Send + 'static> TuiEngine<W> {
    pub fn run<F: FnOnce(&mut Frame) -> Result<()>>(mut self, process: F) {
        let tx = self.tx;
        let handle = self.runtime.spawn();

        let procs =
            process(&mut self.frame).and_then(|_| tx.send(RuntimeTask::Close).map_err(Error::Send));

        if let Err(e) = procs {
            handle_err(e);
        }

        handle.join().ok();
    }

    pub fn run_lazy<F: FnOnce(&mut Frame) -> Result<()> + Send + 'static>(
        mut self,
        process: F,
    ) -> impl Future<Output = ()> {
        let tx = self.tx;
        let handle = self.runtime.spawn();

        async move {
            let procs = process(&mut self.frame)
                .and_then(|_| tx.send(RuntimeTask::Close).map_err(Error::Send));

            if let Err(e) = procs {
                handle_err(e);
            }

            handle.join().ok();
        }
    }
}

impl Default for TuiEngine<Stdout> {
    fn default() -> Self {
        let config = TuiConfig::default();
        let base_pos = config.base_pos;
        let config = config.into();
        let renderer = TerminalRenderer::new(io::stdout());
        let (runtime, tx) = RenderRuntime::new(config, renderer);
        let frame = Frame::new(base_pos, tx.clone());

        Self { runtime, frame, tx }
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

pub struct Frame {
    window: Window,
    tx: Sender<RuntimeTask>,
}

impl Frame {
    fn new(base_pos: (u16, u16), tx: Sender<RuntimeTask>) -> Self {
        Self {
            window: Window::new(base_pos.0, base_pos.1),
            tx,
        }
    }

    fn create_vterm(&mut self) -> VTerm {
        let new_window = Window::new(self.window.width(), self.window.height());
        let ejected = mem::replace(&mut self.window, new_window);

        ejected.into_vterm()
    }

    pub fn overlap(&mut self, view: Annot<View>) {
        self.window.overlap(view);
    }

    pub fn update_frame(&mut self) -> Result<()> {
        let vterm = self.create_vterm();

        self.tx
            .send(RuntimeTask::UpdateBuffer(vterm))
            .map_err(Error::Send)
    }
}
