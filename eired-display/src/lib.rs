mod annot;
mod cell;
mod span;
mod style;

use std::fmt::Debug;

pub use annot::{Annot, Annotate};
pub use cell::Cell;
pub use span::{Span, VisualSpan};
pub use style::{AnsiColor, Color, Style};

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
