use std::fmt::Debug;
use std::slice::Iter;
use std::vec::IntoIter;

use crate::{Annotate, Cell};

#[derive(PartialEq, Eq)]
/// An immutable list of cells for terminal area.
///
/// This is the wrapper struct of [`Vec<Option<Cell>>`].  
/// This only uses to the [Window](crate::Window) for hold the rectangle area.
///
/// # Examples
///
/// Create by canvas:
/// ```
/// use eired_display::Canvas;
/// use eired_display::Layer;
/// use eired_display::Annotate;
/// use eired_display::Span;
/// use eired_display::Cell;
///
/// let mut canvas = Canvas::default();
/// let mut layer = Layer::default();
///
/// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
/// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
///
/// layer.push_span_write(Span::from("O").annotate((1, 0)));
/// layer.push_span_write(Span::from("OOO").annotate((0, 1)));
/// layer.push_span_write(Span::from("O").annotate((1, 2)));
///
/// layer.push_span_write(Span::from(" ").annotate((1, 1)));
///
/// canvas.overlap_layer(layer.annotate((0, 0)));
///
/// let view = canvas.create_view();
///
/// // View's iterator
/// let mut view_iter = view.into_iter();
///
/// // Line 0
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
///
/// // Line 1
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new(' '))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
///
/// // Line 2
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
///
/// // End
/// assert_eq!(view_iter.next(), None);
/// ```
pub struct View {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl View {
    /// Create new struct.
    ///
    /// Wraps [`Vec<Option<Cell>>`] to a View.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::View;
    /// let min = View::new(1, 1, vec![None]);
    ///
    /// assert_eq!(min.iter().next(), Some(&None));
    /// ```
    pub fn new(width: u16, height: u16, cells: Vec<Option<Cell>>) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns cell list length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::View;
    /// let view = View::new(1, 1, vec![None]);
    ///
    /// assert_eq!(view.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` was inner is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::View;
    /// let view = View::new(0, 1, vec![]);
    ///
    /// assert!(view.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get inner iter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::View;
    /// use eired_display::Cell;
    ///
    /// let view = View::new(2, 2, vec![None, None, Some(Cell::new('A')), None]);
    ///
    /// let mut iter = view.iter();
    ///
    /// assert_eq!(iter.next(), Some(&None));
    /// assert_eq!(iter.next(), Some(&None));
    /// assert_eq!(iter.next(), Some(&Some(Cell::new('A'))));
    /// assert_eq!(iter.next(), Some(&None));
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, Option<Cell>> {
        self.cells.iter()
    }

    /// Get slice from `row`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::View;
    /// use eired_display::Cell;
    ///
    /// let view = View::new(2, 2, vec![None, None, Some(Cell::new('A')), None]);
    ///
    /// assert_eq!(view.get_line(0), &[None, None]);
    /// assert_eq!(view.get_line(1), &[Some(Cell::new('A')), None]);
    /// ```
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

impl Clone for View {
    fn clone(&self) -> Self {
        Self {
            height: self.height,
            width: self.width,
            cells: self.cells.clone(),
        }
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
