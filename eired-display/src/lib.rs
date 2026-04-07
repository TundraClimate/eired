//! `eired-display` is low-layered terminal drawing crate.
//!
//! A core sturct [`DrawableSpan`] is draw text to [`io::Write`](std::io::Write) with specified
//! start location.
//! And this crate supply the helper sturcts [`Span`], [`Layer`], [`Canvas`], [`View`], [`Window`]
//! and [`VTerm`] for easily construct the multiple `DrawableSpan`.
//!
//! # Examples
//!
//! ## DrawableSpan
//!
//! ```no_run
//! use eired_display::DrawableSpan;
//! use eired_display::Span;
//!
//! use std::io;
//! use std::io::Write;
//!
//! # fn main() -> io::Result<()> {
//! let cmd = DrawableSpan::new((0, 0), Span::from("Hello, World!").to_vec());
//! let mut stdout = io::stdout();
//!
//! // Draws text "Hello, World!" at (0, 0)
//! cmd.draw(&mut stdout)?;
//!
//! stdout.flush()?;
//! # Ok(())
//! # }
//! ```

mod annot;
mod canvas;
/* mod cell; */
mod draw;
mod layer;
mod span;
mod style;
mod view;
mod window;

use std::fmt::{Debug, Display};

pub use annot::{Annot, Annotate};
pub use canvas::Canvas;
/* pub use cell::Cell; */
pub use draw::{DrawableSpan, convert_to_draws};
pub use layer::Layer;
pub use span::Span;
pub use style::{AnsiColor, Color, Style};
pub use view::View;
pub use window::{VTerm, Window, create_virtual_terminal};

#[derive(PartialEq, Eq)]
/// A marker struct that represents area.
///
/// # Examples
///
/// ```
/// # use eired_display::Rect;
/// use eired_display::Annotate;
///
/// // A rectangle area of (50, 20)
/// let rect = Rect(50, 20);
///
/// assert_eq!(rect.width(), 50);
/// assert_eq!(rect.height(), 20);
/// ```
pub struct Rect(
    /// A width of rectangle.
    pub u16,
    /// A height of rectangle.
    pub u16,
);

impl Rect {
    /// Create new rect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Rect;
    /// use eired_display::Annotate;
    ///
    /// // A rectangle area of (50, 20)
    /// let rect = Rect::new(50, 20);
    ///
    /// assert_eq!(rect.width(), 50);
    /// assert_eq!(rect.height(), 20);
    /// ```
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    ch: char,
    style: Style,
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
