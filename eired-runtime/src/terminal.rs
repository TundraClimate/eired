use std::io::{self, Write};
use std::panic;

use crossterm::cursor::{self, Hide, MoveTo, Show};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};

use crate::config::RuntimeConfig;
use crate::renderer::{Diff, Renderer};

fn enter_alternate<W: Write>(w: &mut W) -> io::Result<()> {
    execute!(w, EnterAlternateScreen)
}

fn leave_alternate<W: Write>(w: &mut W) -> io::Result<()> {
    execute!(w, LeaveAlternateScreen)
}

fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()
}

fn disable_raw_mode() -> io::Result<()> {
    terminal::disable_raw_mode()
}

fn show_cursor<W: Write>(w: &mut W) -> io::Result<()> {
    queue!(w, Show)
}

fn hide_cursor<W: Write>(w: &mut W) -> io::Result<()> {
    queue!(w, Hide)
}

pub fn cursor_pos() -> io::Result<(u16, u16)> {
    cursor::position()
}

pub fn cursor_move_to<W: Write>(w: &mut W, at_x: u16, at_y: u16) -> io::Result<()> {
    queue!(w, MoveTo(at_x, at_y))
}

pub fn get_size() -> (u16, u16) {
    terminal::size().unwrap_or((0, 0))
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
        let cursor = diff.cursor();
        let cmds = eired_display::convert_to_draws(config.base_pos, diff.into_vec());

        for cmd in cmds {
            cmd.draw(&mut self.writer)?;
        }

        if let Some(cursor) = cursor {
            if cursor.is_visible() {
                let moveto = cursor.get_at();

                cursor_move_to(&mut self.writer, moveto.0, moveto.1)?;

                show_cursor(&mut self.writer)?;
            } else {
                hide_cursor(&mut self.writer)?;
            }
        }

        self.writer.flush()
    }

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()> {
        install_panic_hook();

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

fn install_panic_hook() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        let _ = force_restore_terminal();

        default_hook(info);
    }));
}

fn force_restore_terminal() -> io::Result<()> {
    let mut stderr = io::stderr();

    leave_alternate(&mut stderr)?;

    disable_raw_mode()?;

    Ok(())
}
