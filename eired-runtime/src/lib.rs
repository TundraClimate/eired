#![allow(dead_code)]

pub mod config;
pub mod renderer;
pub mod task;
pub mod terminal;

use std::io::Write;
use std::marker::PhantomData;
use std::thread::{self, JoinHandle};

use crossbeam::channel::{self, Receiver, Sender, select};

use config::RuntimeConfig;
use renderer::{RenderOptimizer, Renderer};
use task::{RuntimeTask, TaskContext};

use eired_display::{Annot, Annotate, Cell, Rect, Span};

/// A runtime of renderer the standalone rendering thread.
///
/// The [`new`](Self::new) function returns `(Self, Sender<RuntimeTask>)`.
/// [`Sender`] receives the [`RuntimeTask`], it's includes the `UpdateBuffer` and etc.
///
/// An optimizer as a difference extractor analyze received buffer the
/// [`VTerm`](eired_display::VTerm).
/// The useless rendering process will suppressing for optimize.
///
/// # Examples
///
/// - The [`run`](Self::run) function is start runtime on that thread.
/// - The [`spawn`](Self::spawn) function is start runtime with new thread.
///
/// ```no_run
/// use eired_runtime::RenderRuntime;
/// use eired_runtime::terminal::TerminalRenderer;
/// use eired_runtime::config::ConfigBuilder;
/// use eired_runtime::task::RuntimeTask;
///
/// let renderer = TerminalRenderer::new(std::io::stdout());
/// let config = ConfigBuilder::default().raw_mode(true).alternate_screen(true).fps(60).build();
/// let (runtime, tx) = RenderRuntime::new(config, renderer);
///
/// let handle = runtime.spawn();
///
/// # /*
/// tx.send(RuntimeTask::UpdateBuffer(/* Buffer */)).ok();
/// # */
/// tx.send(RuntimeTask::ClearBuffer).ok();
/// tx.send(RuntimeTask::Close).ok();
///
/// handle.join().ok();
/// ```
pub struct RenderRuntime<W: Write, R: Renderer<W>> {
    out: PhantomData<W>,
    config: RuntimeConfig,
    renderer: R,
    optimizer: RenderOptimizer,
    rx: Receiver<RuntimeTask>,
}

impl<W: Write, R: Renderer<W>> RenderRuntime<W, R> {
    /// Create a new runtime.
    ///
    /// This function returns `(Self, Sender<RuntimeTask>)`.
    /// [`Sender`] receives the [`RuntimeTask`], it's includes the `UpdateBuffer` and etc.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use eired_runtime::RenderRuntime;
    /// use eired_runtime::terminal::TerminalRenderer;
    /// use eired_runtime::config::ConfigBuilder;
    ///
    /// let renderer = TerminalRenderer::new(std::io::stdout());
    /// let config = ConfigBuilder::default().build();
    ///
    /// let (runtime, tx) = RenderRuntime::new(config, renderer);
    /// ```
    pub fn new(config: RuntimeConfig, renderer: R) -> (Self, Sender<RuntimeTask>) {
        let (tx, rx) = channel::bounded(1024);

        let rt = Self {
            out: PhantomData,
            config,
            renderer,
            optimizer: RenderOptimizer::new(),
            rx,
        };

        (rt, tx)
    }

    /// Runs runtime on that thread.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use eired_runtime::RenderRuntime;
    /// # use eired_runtime::terminal::TerminalRenderer;
    /// # use eired_runtime::config::ConfigBuilder;
    /// # use eired_runtime::task::RuntimeTask;
    /// #
    /// # let renderer = TerminalRenderer::new(std::io::stdout());
    /// # let config = ConfigBuilder::default().build();
    /// #
    /// # /*
    /// let (runtime, _) = RenderRuntime::new(/* config, renderer */);
    /// # */
    /// # let (runtime, _) = RenderRuntime::new(config, renderer);
    ///
    /// runtime.run();
    /// ```
    pub fn run(mut self) {
        self.store();
        self.change_loop();
        self.restore();
    }

    fn change_loop(&mut self) {
        let tick = self.config.ticker.clone();

        let mut running = true;
        let mut cursor_vis = false;
        let mut buffer = None;
        let mut cursor = Some(terminal::cursor_pos().unwrap_or((0, 0)));
        let mut cursor_diff = None;
        let mut diff = None;

        while running {
            match tick {
                Some(ref tick) => {
                    select! {
                        recv(self.rx) -> task => {
                            let ctx = TaskContext {
                                buffer: &mut buffer,
                                cursor: &mut cursor,
                                cursor_vis: &mut cursor_vis,
                                running: &mut running,
                            };

                            RuntimeTask::eval(task, ctx);
                        }

                        recv(tick) -> _ => {
                            if let Some(ref cursor) = cursor {
                                cursor_diff = self.optimizer.create_cursor_diff(*cursor, cursor_vis);
                            }

                            if let Some(ref buffer) = buffer {
                                diff = self.optimizer.create_diff(buffer, cursor_diff);
                            }
                        }
                    }
                }
                None => {
                    let task = self.rx.recv();

                    let ctx = TaskContext {
                        buffer: &mut buffer,
                        cursor: &mut cursor,
                        cursor_vis: &mut cursor_vis,
                        running: &mut running,
                    };

                    RuntimeTask::eval(task, ctx);

                    if let Some(ref cursor) = cursor {
                        cursor_diff = self.optimizer.create_cursor_diff(*cursor, cursor_vis);
                    }

                    if let Some(ref buffer) = buffer {
                        diff = self.optimizer.create_diff(buffer, cursor_diff);
                    }
                }
            }

            if let Some(diff) = diff.take() {
                running = self.renderer.render(&self.config, diff).is_ok();

                self.optimizer
                    .replace_cache(buffer.take(), cursor_diff.take());
            }
        }
    }

    fn store(&mut self) {
        self.renderer.store(&self.config).ok();
    }

    fn restore(&mut self) {
        self.renderer.restore(&self.config).ok();
    }
}

impl<W, R> RenderRuntime<W, R>
where
    W: Write + Send + 'static,
    R: Renderer<W> + Send + 'static,
{
    /// Runs runtime on new thread.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use eired_runtime::RenderRuntime;
    /// # use eired_runtime::terminal::TerminalRenderer;
    /// # use eired_runtime::config::ConfigBuilder;
    /// # use eired_runtime::task::RuntimeTask;
    /// #
    /// # let renderer = TerminalRenderer::new(std::io::stdout());
    /// # let config = ConfigBuilder::default().build();
    /// #
    /// # /*
    /// let (runtime, _) = RenderRuntime::new(/* config, renderer */);
    /// # */
    /// # let (runtime, _) = RenderRuntime::new(config, renderer);
    ///
    /// let handle = runtime.spawn();
    /// ```
    pub fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || self.run())
    }
}

pub struct Canvas {
    width: u16,
    height: u16,
    inner: Span,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            inner: Span::from_iter(vec![Cell::default(); (width * height) as usize]),
        }
    }

    pub fn draw<T: Annotate + Into<Vec<Cell>>>(&mut self, paint: Annot<T>) {
        if paint.has_zero() {
            return;
        }

        let paint_base = paint.base();
        let (paint_width, paint_height) = paint.get_size();
        let mut paint = paint.into_inner().into();

        let rect = Rect::new(self.width, self.height).annotate((0, 0));

        if !rect.contains(paint_base) || paint.len() != (paint_width * paint_height) as usize {
            return;
        }

        let truncated_width = paint_width.min(self.width - paint_base.cols()) as usize;
        let truncated_height = paint_height.min(self.height - paint_base.rows()) as usize;

        let dest_slice = self.inner.as_mut_slice();

        for r in 0..truncated_height {
            let dest_grand_pads = (self.width * (paint_base.rows() + r as u16)) as usize;

            let src_row_pads = r * paint_width as usize;
            let dst_row_pads = dest_grand_pads + paint_base.cols() as usize;

            let src = &mut paint[src_row_pads..src_row_pads + truncated_width];
            let dst = &mut dest_slice[dst_row_pads..dst_row_pads + truncated_width];

            src.swap_with_slice(dst);
        }
    }
}

impl Annotate for Canvas {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
