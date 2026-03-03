mod terminal;

use std::io::{self, Write};
use std::marker::PhantomData;

use crossbeam::channel::{self, Receiver, Sender};

use eired_display::{Annotate, VTerm};

pub trait Renderer<W: Write> {
    fn render(&mut self, config: &RuntimeConfig, cells: VTerm) -> io::Result<()>;

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()>;

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()>;
}

pub struct TerminalRenderer<W: Write> {
    writer: W,
}

impl<W: Write> TerminalRenderer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> Renderer<W> for TerminalRenderer<W> {
    fn render(&mut self, config: &RuntimeConfig, cells: VTerm) -> io::Result<()> {
        let cmds = eired_display::convert_to_spans(cells.annotate(config.base_pos));

        for cmd in cmds {
            cmd.draw(&mut self.writer)?;
        }

        self.writer.flush()
    }

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()> {
        if config.alternate_screen {
            terminal::enter_alternate(&mut self.writer)?;
        }

        if config.raw_mode {
            terminal::enable_raw_mode()?;
        }

        Ok(())
    }

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()> {
        if config.alternate_screen {
            terminal::leave_alaternate(&mut self.writer)?;
        }

        if config.raw_mode {
            terminal::disable_raw_mode()?;
        }

        Ok(())
    }
}

pub struct RenderOptimizer {
    prev_cache: Option<VTerm>,
}

impl RenderOptimizer {
    fn replace_cache(&mut self, new_cache: VTerm) {
        self.prev_cache = Some(new_cache);
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

#[derive(Clone, Copy)]
pub struct RuntimeConfig {
    fps: u16,
    pub base_pos: (u16, u16),
    pub alternate_screen: bool,
    pub raw_mode: bool,
}

impl RuntimeConfig {
    fn get_fps_ms(&self) -> f64 {
        1.0 / self.fps as f64
    }
}

pub trait RuntimeTask {}

pub struct GuardHook {
    config: RuntimeConfig,
}

impl GuardHook {
    fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }
}

impl Drop for GuardHook {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        if self.config.alternate_screen {
            terminal::leave_alaternate(&mut stdout).ok();
        }

        if self.config.raw_mode {
            terminal::disable_raw_mode().ok();
        }
    }
}

pub struct RenderRuntime<W: Write, R: Renderer<W>> {
    out: PhantomData<W>,
    config: RuntimeConfig,
    renderer: R,
    optimizer: RenderOptimizer,
    rx: Receiver<Box<dyn RuntimeTask>>,
}

impl<W: Write, R: Renderer<W>> RenderRuntime<W, R> {
    pub fn new(config: RuntimeConfig, renderer: R) -> (Self, Sender<Box<dyn RuntimeTask>>) {
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
        let _hook = GuardHook::new(self.config);

        self.store();
        self.change_loop();
        self.restore();
    }

    fn change_loop(&mut self) {
        todo!();
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
