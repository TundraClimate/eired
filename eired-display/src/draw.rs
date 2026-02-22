use std::fmt::Debug;
use std::io::{self, Stdout};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, Stylize};

use crate::{Annot, Cell};

#[derive(PartialEq, Eq)]
/// A drawing command.
pub struct DrawableSpan {
    moveto: (u16, u16),
    span: Vec<Cell>,
}

impl DrawableSpan {
    /// Create new cmd.
    pub fn new<T: IntoIterator<Item = Cell>>(moveto: (u16, u16), cells: T) -> Self {
        Self {
            moveto,
            span: cells.into_iter().collect::<Vec<_>>(),
        }
    }

    /// Apply styles by crossterm.
    pub fn styled_content(&self) -> String {
        self.span.iter().fold("".to_string(), |acc, cell| {
            let cell = cell.ch.with(cell.fg).on(cell.bg);

            format!("{}{}", acc, cell)
        })
    }

    /// Draws self for `stdout`.
    pub fn draw(&self, stdout: &mut Stdout) -> io::Result<()> {
        draw(stdout, self)
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

fn draw(stdout: &mut Stdout, cmd: &DrawableSpan) -> io::Result<()> {
    let styled = cmd.styled_content();

    queue!(stdout, MoveTo(cmd.moveto.0, cmd.moveto.1), Print(styled))
}
