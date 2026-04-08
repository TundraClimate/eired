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

/* mod annot; */
mod canvas;
/* mod cell; */
mod draw;
mod layer;
/* mod span; */
mod style;
mod view;
mod window;

use std::fmt::{Debug, Display};

/* pub use annot::{Annot, Annotate}; */
pub use canvas::Canvas;
/* pub use cell::Cell; */
pub use draw::{DrawableSpan, convert_to_draws};
pub use layer::Layer;
/* pub use span::Span; */
pub use style::{AnsiColor, Color, Style};
pub use view::View;
pub use window::{VTerm, Window, create_virtual_terminal};

pub struct Annot<T> {
    base: Point,
    inner: T,
}

impl<T> Annot<T> {
    pub fn new<P: Into<Point>>(root: P, inner: T) -> Self {
        Self {
            base: root.into(),
            inner,
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn rebase<F: Fn(&mut u16, &mut u16)>(&mut self, f: F) {
        f(&mut self.base.0, &mut self.base.1);
    }
}

impl<T: Annotate> Annot<T> {
    pub fn width(&self) -> u16 {
        self.inner.width()
    }

    pub fn height(&self) -> u16 {
        self.inner.height()
    }

    pub fn get_size(&self) -> (u16, u16) {
        self.inner.get_size()
    }

    pub fn has_zero(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    pub fn base(&self) -> Point {
        self.base
    }

    pub fn in_bound(&self) -> Point {
        Point::from((
            self.base.cols() + self.width().max(1) - 1,
            self.base.rows() + self.height().max(1) - 1,
        ))
    }

    pub fn out_bound(&self) -> Point {
        Point::from((
            self.base.cols() + self.width(),
            self.base.rows() + self.height(),
        ))
    }

    pub fn is_conflict<A: Annotate>(&self, other: &Annot<A>) -> bool {
        if self.has_zero() || other.has_zero() {
            return false;
        }

        let self_base = self.base();
        let self_out = self.out_bound();
        let other_base = other.base();
        let other_out = other.out_bound();

        self_out.cols() > other_base.cols()
            && other_out.cols() > self_base.cols()
            && self_out.rows() > other_base.rows()
            && other_out.rows() > self_base.rows()
    }

    pub fn contains<P: Into<Point>>(&self, p: P) -> bool {
        self.is_conflict(&Rect(1, 1).annotate(p))
    }
}

impl<T: Copy> Copy for Annot<T> {}

impl<T: Clone> Clone for Annot<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug> Debug for Annot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} on {:?}", self.inner(), self.base)
    }
}

impl<T: Eq> Eq for Annot<T> {}

impl<T: PartialEq> PartialEq for Annot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.inner == other.inner
    }
}

pub trait Annotate {
    fn annotate<P: Into<Point>>(self, root: P) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16);

    fn width(&self) -> u16 {
        self.get_size().0
    }

    fn height(&self) -> u16 {
        self.get_size().1
    }
}

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

#[derive(Clone, PartialEq, Eq)]
pub struct Span {
    inner: Vec<Cell>,
}

impl Span {
    pub fn new<S: AsRef<str>>(s: S, style: Style) -> Self {
        Self::from_iter(s.as_ref().chars().map(|c| Cell::new(c, style)))
    }

    pub fn len(&self) -> u16 {
        self.inner.len() as u16
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[Cell] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [Cell] {
        &mut self.inner
    }

    pub fn to_vec(&self) -> Vec<Cell> {
        self.clone().into()
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        write!(
            f,
            "{}",
            self.inner
                .iter()
                .try_fold("".to_string(), |mut acc, cell| {
                    write!(acc, "{:?}", cell)?;

                    Ok(acc)
                })?
        )
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        write!(
            f,
            "{}",
            self.inner
                .iter()
                .try_fold("".to_string(), |mut acc, cell| {
                    write!(acc, "{}", cell)?;

                    Ok(acc)
                })?
        )
    }
}

impl From<String> for Span {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Span {
    fn from(value: &str) -> Self {
        Self::from_iter(value.chars().map(|c| Cell::from(c)))
    }
}

impl FromIterator<Cell> for Span {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect::<Vec<_>>(),
        }
    }
}

impl From<Span> for Vec<Cell> {
    fn from(value: Span) -> Self {
        value.inner
    }
}

impl Annotate for Span {
    fn get_size(&self) -> (u16, u16) {
        (self.len(), 1)
    }
}
