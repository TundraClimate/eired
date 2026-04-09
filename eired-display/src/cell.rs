use std::fmt::{Debug, Display};

use crate::{Annotate, Style};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

impl Cell {
    pub fn new(ch: char, style: Style) -> Self {
        Self { ch, style }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.style, self.ch)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.style, self.ch)
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            style: Style::default(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::from(' ')
    }
}

impl Annotate for Cell {
    fn get_size(&self) -> (u16, u16) {
        (1, 1)
    }
}
