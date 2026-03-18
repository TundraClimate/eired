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
