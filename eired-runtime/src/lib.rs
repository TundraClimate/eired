#![allow(dead_code)]

pub mod config;
pub mod renderer;
pub mod task;
pub mod terminal;

use std::io::Write;
use std::marker::PhantomData;

use crossbeam::channel::{self, Receiver, Sender, select};

use config::RuntimeConfig;
use renderer::{RenderOptimizer, Renderer};
use task::{RuntimeTask, TaskContext};
use terminal::TerminalGuard;

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
        let _guard = TerminalGuard::new(self.config.clone());

        self.store();
        self.change_loop();
        self.restore();
    }

    fn change_loop(&mut self) {
        let tick = self.config.ticker.clone();

        let mut running = true;
        let mut buffer = None;
        let mut diff = None;

        while running {
            match tick {
                Some(ref tick) => {
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
                }
                None => {
                    let task = self.rx.recv();

                    let ctx = TaskContext {
                        buffer: &mut buffer,
                        running: &mut running,
                    };

                    RuntimeTask::eval(task, ctx);

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
        self.renderer.store(&self.config).ok();
    }

    fn restore(&mut self) {
        self.renderer.restore(&self.config).ok();
    }
}
