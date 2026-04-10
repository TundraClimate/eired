use std::fmt::{Debug, Display};
use std::io;
use std::slice::Iter;
use std::vec::IntoIter;

use crate::{Annotate, Cell, Point, Style};

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

    pub fn iter(&self) -> Iter<'_, Cell> {
        self.inner.iter()
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
        Self::from_iter(value.chars().map(Cell::from))
    }
}

impl<'a> IntoIterator for &'a Span {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for Span {
    type Item = Cell;
    type IntoIter = IntoIter<Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VisualSpan<'a> {
    moveto: Point,
    span: &'a [Cell],
}

impl<'a> VisualSpan<'a> {
    pub fn new<P: Into<Point>>(moveto: P, span: &'a [Cell]) -> VisualSpan<'a> {
        Self {
            moveto: moveto.into(),
            span,
        }
    }
}

impl VisualSpan<'_> {
    pub fn draw<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        let fmt = format!(
            "\x1b[{};{}H{}",
            self.moveto.rows() + 1,
            self.moveto.cols() + 1,
            Span::from_iter(self.span.to_vec())
        );

        w.write(fmt.as_bytes()).map(|_| ())
    }
}
