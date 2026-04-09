#![allow(dead_code)]

use std::io::{self, Stdout, Write};
use std::mem;
use std::process;
use std::sync::Arc;

use crossbeam::channel::{SendError, Sender};

pub use eired_display::*;
pub use eired_runtime::*;

use eired_runtime::config::{ConfigBuilder, RuntimeConfig};
use eired_runtime::task::RuntimeTask;
use eired_runtime::terminal::TerminalRenderer;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    Send(SendError<RuntimeTask>),
}

fn handle_err(err: Error) {
    match err {
        Error::Io(_) => process::exit(5),
        Error::Send(_) => process::exit(1),
    }
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
    base_pos: Point,
    size: Arc<dyn Fn() -> (u16, u16)>,
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

pub struct Frame {
    canvas: Canvas,
    size_fn: Arc<dyn Fn() -> (u16, u16)>,
    tx: Sender<RuntimeTask>,
    width: u16,
    height: u16,
    cursor: Option<Point>,
    cursor_vis: Option<bool>,
}

impl Frame {
    fn new(size: Arc<dyn Fn() -> (u16, u16)>, tx: Sender<RuntimeTask>) -> Self {
        let terminal_size = size();
        let width = terminal_size.0;
        let height = terminal_size.1;

        Self {
            canvas: Canvas::new(width, height),
            size_fn: size,
            tx,
            width,
            height,
            cursor: None,
            cursor_vis: None,
        }
    }

    fn take_canvas(&mut self) -> Canvas {
        self.on_resize();

        let new_canvas = Canvas::new(self.width, self.height);

        mem::replace(&mut self.canvas, new_canvas)
    }

    pub fn on_resize(&mut self) {
        let terminal_size = (self.size_fn)();

        if (self.width, self.height) != terminal_size {
            self.width = terminal_size.0;
            self.height = terminal_size.1;
            self.canvas = Canvas::new(self.width, self.height);
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cursor_move_to<P: Into<Point>>(&mut self, at: P) {
        self.cursor = Some(at.into());
    }

    pub fn show_cursor(&mut self) {
        self.cursor_vis = Some(true);
    }

    pub fn hide_cursor(&mut self) {
        self.cursor_vis = Some(false);
    }

    pub fn draw<T: Annotate + Into<Vec<Cell>>>(&mut self, cells: Annot<T>) {
        self.canvas.draw(cells);
    }

    pub fn update_frame(&mut self) -> Result<()> {
        let canvas = self.take_canvas();

        if let Some(vis) = self.cursor_vis.take() {
            if vis {
                self.tx.send(RuntimeTask::ShowCursor).map_err(Error::Send)?;

                if let Some(cursor) = self.cursor.take() {
                    self.tx
                        .send(RuntimeTask::MoveCursor(cursor))
                        .map_err(Error::Send)?;
                }
            } else {
                self.tx.send(RuntimeTask::HideCursor).map_err(Error::Send)?;
                self.tx
                    .send(RuntimeTask::MoveCursor(Point::default()))
                    .map_err(Error::Send)?;
            }
        }

        self.tx
            .send(RuntimeTask::UpdateBuffer(canvas))
            .map_err(Error::Send)
    }
}
