use std::fmt::Debug;

use crossterm::style::Color;

use crate::Annotate;

#[derive(Clone, Copy, PartialEq, Eq)]
/// A struct of corresponds terminal 1 pixel.
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    /// Create new cell.
    pub fn new(ch: char) -> Self {
        Self::from(ch)
    }

    /// Create new cell with foreground.
    pub fn new_fg(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            ..Self::default()
        }
    }

    /// Create new cell with background.
    pub fn new_bg(ch: char, bg: Color) -> Self {
        Self {
            ch,
            bg,
            ..Self::default()
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            ..Self::default()
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("ch", &self.ch)
            .field("fg", &self.fg)
            .field("bg", &self.bg)
            .finish()
    }
}

impl Annotate for Cell {
    fn get_size(&self) -> (u16, u16) {
        (1, 1)
    }
}
