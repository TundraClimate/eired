use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;

use crossterm::style::Color;

use crate::{Annotate, Cell};

#[derive(Default, PartialEq, Eq)]
/// A list wrapper of lined cells.
pub struct Span {
    len: u16,
    cells: VecDeque<Cell>,
}

impl Span {
    /// Create new span with background.
    pub fn new_with_bg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.bg = color);

        span
    }

    /// Create new span with foreground.
    pub fn new_with_fg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.fg = color);

        span
    }

    /// Get 1 cell ref by `idx`.
    pub fn get(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    /// Get 1 cell ref mut by `idx`.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    /// Replaces at `idx` cell.
    pub fn replace_at(&mut self, idx: usize, cell: Cell) -> Option<Cell> {
        self.get_mut(idx).map(|before| mem::replace(before, cell))
    }

    /// Returns span length.
    pub fn len(&self) -> u16 {
        self.len
    }

    /// Returns `true` was length is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes cell to back of span.
    pub fn push_back(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_back(cell);
    }

    /// Pushes cell to front of span.
    pub fn push_front(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_front(cell);
    }

    /// Pops cell from back of span.
    pub fn pop_back(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_back()
    }

    /// Pops cell from front of span.
    pub fn pop_front(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_front()
    }

    /// Truncates `num` cells to front.
    pub fn truncate_front(&mut self, num: u16) {
        let tmp = self.cells.drain(num as usize..).collect();

        self.len = self.len().max(num) - num;
        self.cells = tmp;
    }

    /// Truncates `num` cells to back.
    pub fn truncate_back(&mut self, num: u16) {
        self.len = self.len().max(num) - num;
        self.cells.truncate(self.len() as usize);
    }

    /// Returns copied span to [Vec].
    pub fn to_vec(&self) -> Vec<Cell> {
        self.cells.iter().copied().collect()
    }

    /// Returns span parts by split indecies.
    pub fn split_by(&self, indecies: &[u16]) -> Vec<Option<Span>> {
        debug_assert!(indecies.is_sorted(), "indecies not sorted");

        let mut res = vec![];
        let mut i = 0usize;
        let cells = self.to_vec();

        for j in indecies {
            let j = *j as usize;

            res.push(cells.get(i..j).map(|c| Span::from_iter(c.iter().copied())));

            i = j;
        }

        res.push(cells.get(i..).map(|c| Span::from_iter(c.iter().copied())));

        res
    }
}

impl Clone for Span {
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            cells: self.cells.clone(),
        }
    }
}

impl From<&str> for Span {
    fn from(value: &str) -> Self {
        let mut span = Self::default();

        for c in value.chars() {
            span.push_back(Cell::new(c));
        }

        span
    }
}

impl From<String> for Span {
    fn from(value: String) -> Self {
        let mut span = Self::default();

        for c in value.chars() {
            span.push_back(Cell::new(c));
        }

        span
    }
}

impl FromIterator<Cell> for Span {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut span = Self::default();

        for c in iter.into_iter() {
            span.push_back(c);
        }

        span
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("len", &self.len)
            .field("cells", &self.cells.iter().enumerate().collect::<Vec<_>>())
            .finish()
    }
}

impl Annotate for Span {
    fn get_size(&self) -> (u16, u16) {
        (self.len(), 1)
    }
}
