use std::fmt::Debug;

use crate::terminal::{Annotate, Cell};
use crate::widget::{Span, Widget};

#[derive(Clone, PartialEq, Eq)]
pub struct Layer {
    inner: Span,
    width: u16,
    height: u16,
}

impl Layer {
    pub fn with_lines<C, T>(lines: T) -> Option<Self>
    where
        C: IntoIterator<Item = Cell>,
        T: IntoIterator<Item = C>,
    {
        let mut height = 0usize;
        let cells = lines
            .into_iter()
            .enumerate()
            .inspect(|(i, _)| height = *i + 1)
            .flat_map(|(_, c)| c)
            .collect::<Vec<_>>();

        if height == 0 {
            return None;
        }

        (cells.len() % height == 0).then_some(Self {
            width: (cells.len() / height) as u16,
            height: height as u16,
            inner: Span::from_iter(cells),
        })
    }

    pub fn with_size<T: IntoIterator<Item = Cell>>(size: (u16, u16), cells: T) -> Option<Self> {
        let cells = cells.into_iter().collect::<Vec<_>>();
        let (width, height) = size;

        ((width * height) as usize == cells.len()).then_some(Self {
            width,
            height,
            inner: Span::from_iter(cells),
        })
    }

    pub fn as_slice(&self) -> &[Cell] {
        self.inner.as_slice()
    }

    pub fn line(&self, line: u16) -> Option<&[Cell]> {
        let start = (self.width * line) as usize;
        let end = start + self.width as usize;

        (self.height > line).then_some(&self.inner.as_slice()[start..end])
    }
}

impl Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];

        for i in 0..self.height {
            let Some(line) = self.line(i) else {
                continue;
            };

            let span = Span::from_iter(line.iter().copied());

            lines.push(format!("{:?}", span));
        }

        write!(f, "[{}]", lines.join(","))
    }
}

impl Annotate for Layer {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

impl Widget for Layer {
    fn into_cells(self) -> Vec<Cell> {
        self.inner.into()
    }
}
