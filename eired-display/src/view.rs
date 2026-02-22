use std::fmt::Debug;
use std::slice::Iter;
use std::vec::IntoIter;

use crate::{Annotate, Cell};

#[derive(PartialEq, Eq)]
/// An immutable list of cells for terminal area.
pub struct View {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl View {
    /// Create new struct.
    pub fn new(width: u16, height: u16, cells: Vec<Option<Cell>>) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns cell list length.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` was inner is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get inner iter.
    pub fn iter<'a>(&'a self) -> Iter<'a, Option<Cell>> {
        self.cells.iter()
    }

    /// Get slice from `row`.
    pub fn get_line(&self, rows: u16) -> &[Option<Cell>] {
        if rows >= self.height {
            return &[];
        }

        let start = (self.width * rows) as usize;
        let end = (self.width * (rows + 1)) as usize;

        &self.cells[start..end]
    }
}

impl IntoIterator for View {
    type Item = Option<Cell>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<'a> IntoIterator for &'a View {
    type Item = &'a Option<Cell>;
    type IntoIter = Iter<'a, Option<Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

impl Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("View")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("cells", &self.cells)
            .finish()
    }
}

impl Annotate for View {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
