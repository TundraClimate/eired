#![allow(dead_code)]

use std::io::{self, Stdout, Write};
use std::mem;
use std::sync::Arc;

use crossbeam::channel::{SendError, Sender};

use eired_display::{Annot, VTerm, View, Window};
use eired_runtime::RenderRuntime;
use eired_runtime::config::{ConfigBuilder, RuntimeConfig};
use eired_runtime::task::RuntimeTask;
use eired_runtime::terminal::{self, TerminalRenderer};

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
        let size = config.size.clone();
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
    base_pos: (u16, u16),
    size: Arc<dyn Fn() -> (u16, u16)>,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            raw_mode: true,
            alternate_screen: true,
            fps: 30,
            base_pos: (0, 0),
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

pub struct Frame {
    window: Window,
    size: Arc<dyn Fn() -> (u16, u16)>,
    tx: Sender<RuntimeTask>,
}

impl Frame {
    fn new(size: Arc<dyn Fn() -> (u16, u16)>, tx: Sender<RuntimeTask>) -> Self {
        let terminal_size = size();

        Self {
            window: Window::new(terminal_size.0, terminal_size.1),
            size,
            tx,
        }
    }

    fn create_vterm(&mut self) -> VTerm {
        let terminal_size = (self.size)();
        let new_window = Window::new(terminal_size.0, terminal_size.1);
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
