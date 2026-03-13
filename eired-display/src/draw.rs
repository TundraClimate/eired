use std::fmt::Debug;
use std::io::{self, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, Stylize};

use crate::{Annot, Cell, Span};

#[derive(PartialEq, Eq)]
/// A drawing command.
///
/// Includes a start position and span.
/// Calls the [`draw`](DrawableSpan::draw) function to draws span to the terminal with rendering crate. (e.g. [crossterm])  
///
/// # Notes
///
/// In the current version, only support the [`crossterm`] to draw.  
/// This may change in the future.
///
/// # Examples
///
/// ```no_run
/// # use eired_display::DrawableSpan;
/// use eired_display::Span;
/// use std::io;
/// use std::io::Write;
///
/// let cmd = DrawableSpan::new((0, 0), Span::from("Hello, World!").to_vec());
/// let mut stdout = io::stdout();
///
/// cmd.draw(&mut stdout).ok();
///
/// stdout.flush().ok();
/// ```
pub struct DrawableSpan {
    moveto: (u16, u16),
    span: Vec<Cell>,
}

impl DrawableSpan {
    /// Create new cmd.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use eired_display::DrawableSpan;
    /// use eired_display::Span;
    ///
    /// DrawableSpan::new((0, 0), Span::from("Hello, World!").to_vec());
    /// ```
    pub fn new<T: IntoIterator<Item = Cell>>(moveto: (u16, u16), cells: T) -> Self {
        Self {
            moveto,
            span: cells.into_iter().collect::<Vec<_>>(),
        }
    }

    /// Returns raw text.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use eired_display::DrawableSpan;
    /// use eired_display::Span;
    ///
    /// let cmd = DrawableSpan::new((0, 0), Span::from("Hello, World!").to_vec());
    ///
    /// println!("{}", cmd.raw_content());
    /// ```
    pub fn raw_content(&self) -> String {
        self.span
            .iter()
            .fold("".to_string(), |acc, c| format!("{acc}{}", c.ch))
    }

    /// Apply styles by crossterm.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use eired_display::DrawableSpan;
    /// use eired_display::Span;
    ///
    /// let cmd = DrawableSpan::new((0, 0), Span::from("Hello, World!").to_vec());
    ///
    /// println!("{}", cmd.styled_content());
    /// ```
    pub fn styled_content(&self) -> String {
        self.span.iter().fold("".to_string(), |acc, cell| {
            let cell = cell.ch.with(cell.fg).on(cell.bg);

            format!("{}{}", acc, cell)
        })
    }

    /// Draws self for `write`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use eired_display::DrawableSpan;
    /// use eired_display::Span;
    /// use std::io;
    /// use std::io::Write;
    ///
    /// let cmd = DrawableSpan::new((0, 0), Span::from("Hello, World!").to_vec());
    /// let mut stdout = io::stdout();
    ///
    /// cmd.draw(&mut stdout).ok();
    ///
    /// stdout.flush().ok();
    /// ```
    pub fn draw<W: Write>(&self, write: &mut W) -> io::Result<()> {
        draw(write, self)
    }
}

impl<T: Iterator<Item = Cell>> From<Annot<T>> for DrawableSpan {
    fn from(value: Annot<T>) -> Self {
        Self::new(value.base_pos(), value.into_inner())
    }
}

impl Debug for DrawableSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Draw")
            .field(
                "MoveTo",
                &format!("(x: {}, y: {})", self.moveto.0, self.moveto.1),
            )
            .field("cells", &self.span)
            .finish()
    }
}

fn draw<W: Write>(write: &mut W, cmd: &DrawableSpan) -> io::Result<()> {
    let styled = cmd.styled_content();

    queue!(write, MoveTo(cmd.moveto.0, cmd.moveto.1), Print(styled))
}

pub fn convert_to_draws(spans: Annot<Vec<Annot<Span>>>) -> Vec<DrawableSpan> {
    let (base_x, base_y) = spans.base_pos();

    spans
        .into_inner()
        .into_iter()
        .map(|mut annot| {
            annot.rebase(|x, y| {
                *x += base_x;
                *y += base_y;
            });

            DrawableSpan::new(annot.base_pos(), annot.into_inner().to_vec())
        })
        .collect()
}
