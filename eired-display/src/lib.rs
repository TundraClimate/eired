//! `eired-display` is low-layered terminal drawing crate.
//!
//! A core struct the [`VisualSpan`] is draw text to [`io::Write`](std::io::Write) with specified
//! start location.
//!
//! The [`VisualSpan`] is execute these process as ansi escape.
//! - Move cursor to expected location.
//! - Writes text that composed by [`Cell`] array to the [`io::Write`](std::io::Write) impls but
//!   not with flush.
//! - Cursor stays at the end of text.
//!
//! ## Note
//!
//! This crate interprets the top left point as `(0, 0)`. since prevent `(0, 0)` and `(1, 1)` result
//! to same by it uses `u16` to specify point for draw.
//!
//! ## Exmaple
//!
//! A simply example:  
//! ```no_run
//! use eired_display::{VisualSpan, Span};
//! use std::io::{self, Write};
//!
//! # fn main() -> io::Result<()> {
//! #
//! let text = Span::from("That's exmaple text.");
//!
//! // Writes at cols:rows = 5:5
//! let vs = VisualSpan::new((5, 5), text.as_slice());
//!
//! let mut stdout = io::stdout();
//!
//! // Draws stdout but not flushed
//! vs.draw(&mut stdout)?;
//!
//! // Flushs stdout
//! stdout.flush()?;
//! # Ok(())
//! # }
//! ```
//!
//! With styles:
//! ```no_run
//! use eired_display::{VisualSpan, Span, Style, AnsiColor};
//! use std::io::{self, Write};
//!
//! # fn main() -> io::Result<()> {
//! #
//! // The text that has red foreground
//! let text = Span::new("That's styled text.", Style::with_fg(AnsiColor::Red));
//!
//! // Writes at cols:rows = 5:5
//! let vs = VisualSpan::new((5, 5), text.as_slice());
//!
//! let mut stdout = io::stdout();
//!
//! // Draws stdout but not flushed
//! vs.draw(&mut stdout)?;
//!
//! // Flushs stdout
//! stdout.flush()?;
//! # Ok(())
//! # }
//! ```

mod annot;
mod cell;
mod span;
mod style;

use std::fmt::Debug;

pub use annot::{Annot, Annotate};
pub use cell::Cell;
pub use span::{Span, VisualSpan};
pub use style::{AnsiColor, Style};

#[derive(PartialEq, Eq)]
pub struct Rect(pub u16, pub u16);

impl Rect {
    pub fn new(width: u16, height: u16) -> Self {
        Self(width, height)
    }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect")
            .field("width", &self.0)
            .field("height", &self.1)
            .finish()
    }
}

impl Annotate for Rect {
    fn get_size(&self) -> (u16, u16) {
        (self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Point(u16, u16);

impl Point {
    pub fn cols(&self) -> u16 {
        self.0
    }

    pub fn rows(&self) -> u16 {
        self.1
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.cols(), self.rows())
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self(value.0, value.1)
    }
}
