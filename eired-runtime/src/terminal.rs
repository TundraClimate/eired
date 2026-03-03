use std::io::{self, Write};

use crossterm::queue;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

pub fn enter_alternate<W: Write>(w: &mut W) -> io::Result<()> {
    queue!(w, EnterAlternateScreen)
}

pub fn leave_alaternate<W: Write>(w: &mut W) -> io::Result<()> {
    queue!(w, LeaveAlternateScreen)
}

pub fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()
}

pub fn disable_raw_mode() -> io::Result<()> {
    terminal::disable_raw_mode()
}
