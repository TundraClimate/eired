mod config;
mod task;
mod terminal;

use std::io::{self, Write};
use std::marker::PhantomData;
use std::time::Duration;

use crossbeam::channel::{self, Receiver, Sender, select};

use eired_display::{Annotate, VTerm};

use config::RuntimeConfig;
use task::{RuntimeTask, TaskContext};
use terminal::TerminalGuard;

pub trait Renderer<W: Write> {
    fn render(&mut self, config: &RuntimeConfig, cells: VTerm) -> io::Result<()>;

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()>;

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()>;
}

struct RenderOptimizer {
    prev_cache: Option<VTerm>,
}

impl RenderOptimizer {
    fn replace_cache(&mut self, new_cache: Option<VTerm>) {
        self.prev_cache = new_cache;
    }

    fn create_diff(&self, new_term: &VTerm) -> Option<VTerm> {
        let Some(ref prev_cache) = self.prev_cache else {
            return Some(new_term.clone());
        };

        if prev_cache.len() != new_term.len() {
            return Some(new_term.clone());
        }

        let mut cells = vec![None; new_term.len()];
        let mut is_changed = false;

        for (i, new_cell) in new_term.iter().enumerate() {
            if prev_cache.get(i) != new_term.get(i) {
                cells[i] = *new_cell;

                if !is_changed {
                    is_changed = true;
                }
            }
        }

        is_changed.then_some(VTerm::new(new_term.width(), new_term.height(), cells))
    }
}

impl RenderOptimizer {
    fn new() -> Self {
        Self { prev_cache: None }
    }
}

pub struct RenderRuntime<W: Write, R: Renderer<W>> {
    out: PhantomData<W>,
    config: RuntimeConfig,
    renderer: R,
    optimizer: RenderOptimizer,
    rx: Receiver<RuntimeTask>,
}

impl<W: Write, R: Renderer<W>> RenderRuntime<W, R> {
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

    pub fn run(mut self) {
        let _guard = TerminalGuard::new(self.config);

        self.store();
        self.change_loop();
        self.restore();
    }

    fn change_loop(&mut self) {
        let tick = channel::tick(Duration::from_secs_f64(self.config.get_fps_tick()));

        let mut running = true;
        let mut buffer = None;
        let mut diff = None;

        while running {
            select! {
                recv(self.rx) -> task => {
                    let ctx = TaskContext {
                        buffer: &mut buffer,
                        running: &mut running,
                    };

                    RuntimeTask::eval(task, ctx);
                }

                recv(tick) -> _ => {
                    if let Some(ref buffer) = buffer {
                        diff = self.optimizer.create_diff(buffer);
                    }
                }
            }

            if let Some(diff) = diff.take() {
                running = self.renderer.render(&self.config, diff).is_ok();

                self.optimizer.replace_cache(buffer.take());
            }
        }
    }

    fn store(&mut self) {
        let config = self.config;

        self.renderer.store(&config).ok();
    }

    fn restore(&mut self) {
        let config = self.config;

        self.renderer.restore(&config).ok();
    }
}
