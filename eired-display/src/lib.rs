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
mod cell;
mod draw;
mod layer;
mod span;
mod view;
mod window;

use std::fmt::Debug;

pub use annot::{Annot, Annotate};
pub use canvas::Canvas;
pub use cell::Cell;
pub use draw::{DrawableSpan, convert_to_draws};
pub use layer::Layer;
pub use span::Span;
pub use view::View;
pub use window::{VTerm, Window, convert_to_spans, create_virtual_terminal};

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
