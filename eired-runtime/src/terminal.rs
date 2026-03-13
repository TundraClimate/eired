use std::io::{self, Write};

use crossterm::queue;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

use eired_display::Annot;

use crate::config::RuntimeConfig;
use crate::renderer::{Diff, Renderer};

fn enter_alternate<W: Write>(w: &mut W) -> io::Result<()> {
    queue!(w, EnterAlternateScreen)
}

fn leave_alternate<W: Write>(w: &mut W) -> io::Result<()> {
    queue!(w, LeaveAlternateScreen)
}

fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()
}

fn disable_raw_mode() -> io::Result<()> {
    terminal::disable_raw_mode()
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
    fn render(&mut self, config: &RuntimeConfig, diff: Diff) -> io::Result<()> {
        let cmds = eired_display::convert_to_draws(Annot::new(config.base_pos, diff.into_vec()));

        for cmd in cmds {
            cmd.draw(&mut self.writer)?;
        }

        self.writer.flush()
    }

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()> {
        if config.alternate_screen {
            enter_alternate(&mut self.writer)?;
        }

        if config.raw_mode {
            enable_raw_mode()?;
        }

        Ok(())
    }

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()> {
        if config.alternate_screen {
            leave_alternate(&mut self.writer)?;
        }

        if config.raw_mode {
            disable_raw_mode()?;
        }

        Ok(())
    }
}

pub(crate) struct TerminalGuard {
    config: RuntimeConfig,
}

impl TerminalGuard {
    pub(crate) fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        if self.config.alternate_screen {
            leave_alternate(&mut stdout).ok();
        }

        if self.config.raw_mode {
            disable_raw_mode().ok();
        }
    }
}
