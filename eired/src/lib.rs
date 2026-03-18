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
    size_fn: Arc<dyn Fn() -> (u16, u16)>,
    tx: Sender<RuntimeTask>,
    width: u16,
    height: u16,
}

impl Frame {
    fn new(size: Arc<dyn Fn() -> (u16, u16)>, tx: Sender<RuntimeTask>) -> Self {
        let terminal_size = size();
        let width = terminal_size.0;
        let height = terminal_size.1;

        Self {
            window: Window::new(width, height),
            size_fn: size,
            tx,
            width,
            height,
        }
    }

    fn create_vterm(&mut self) -> VTerm {
        self.on_resize();

        let new_window = Window::new(self.width, self.height);
        let ejected = mem::replace(&mut self.window, new_window);

        ejected.into_vterm()
    }

    pub fn on_resize(&mut self) {
        let terminal_size = (self.size_fn)();

        if (self.width, self.height) != terminal_size {
            self.width = terminal_size.0;
            self.height = terminal_size.1;
            self.window.resize(self.width, self.height);
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
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
