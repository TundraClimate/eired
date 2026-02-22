use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;

use crossterm::style::Color;

use crate::{Annotate, Cell};

#[derive(Default, PartialEq, Eq)]
/// A list wrapper of lined cells.
///
/// Call the `&mut` functions to modify the inner list, and `&` functions to read inner cells.
///
/// # Examples
///
/// ```
/// # use eired_display::Span;
/// let mut span = Span::from("Hello,");
///
/// span.push_all(" World!");
///
/// assert_eq!(span, Span::from("Hello, World!"));
/// ```
pub struct Span {
    len: u16,
    cells: VecDeque<Cell>,
}

impl Span {
    /// Create new span with background.
    ///
    /// # Note
    ///
    /// `color` is the [`crossterm::style::Color`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html).  
    /// This may change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    /// use crossterm::style::Color;
    ///
    /// let span = Span::new_with_bg("Blue text", Color::Blue);
    ///
    /// assert_eq!(span.get(0), Some(&Cell::new_bg('B', Color::Blue)));
    /// assert_eq!(span.get(8), Some(&Cell::new_bg('t', Color::Blue)));
    /// ```
    pub fn new_with_bg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.bg = color);

        span
    }

    /// Create new span with foreground.
    ///
    /// # Note
    ///
    /// `color` is the [`crossterm::style::Color`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html).  
    /// This may change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    /// use crossterm::style::Color;
    ///
    /// let span = Span::new_with_fg("Red text", Color::Red);
    ///
    /// assert_eq!(span.get(0), Some(&Cell::new_fg('R', Color::Red)));
    /// assert_eq!(span.get(7), Some(&Cell::new_fg('t', Color::Red)));
    /// ```
    pub fn new_with_fg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.fg = color);

        span
    }

    /// Get 1 cell ref by `idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let span = Span::from("Hello, World!");
    ///
    /// assert_eq!(span.get(0), Some(&Cell::new('H')));
    /// assert_eq!(span.get(7), Some(&Cell::new('W')));
    /// ```
    pub fn get(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    /// Get 1 cell ref mut by `idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let mut span = Span::from("Hello. World?");
    ///
    /// span.get_mut(5).unwrap().ch = ',';
    /// span.get_mut(12).unwrap().ch = '!';
    ///
    /// assert_eq!(span.get(5), Some(&Cell::new(',')));
    /// assert_eq!(span.get(12), Some(&Cell::new('!')));
    /// ```
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    /// Replaces at `idx` cell.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let mut span = Span::from("H!, World!");
    ///
    /// span.replace_at(1, Cell::new('i'));
    ///
    /// assert_eq!(span, Span::from("Hi, World!"));
    /// ```
    pub fn replace_at(&mut self, idx: usize, cell: Cell) -> Option<Cell> {
        self.get_mut(idx).map(|before| mem::replace(before, cell))
    }

    /// Returns span length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let span = Span::from("Hello, eired!");
    ///
    /// assert_eq!(span.len(), 13);
    /// ```
    pub fn len(&self) -> u16 {
        self.len
    }

    /// Returns `true` was length is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let span = Span::from("");
    /// let blank = Span::from(" ");
    ///
    /// assert!(span.is_empty());
    /// assert!(!blank.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes cell to back of span.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let mut span = Span::from("");
    ///
    /// span.push_back(Cell::new('H'));
    /// span.push_back(Cell::new('e'));
    /// span.push_back(Cell::new('l'));
    /// span.push_back(Cell::new('l'));
    /// span.push_back(Cell::new('o'));
    ///
    /// assert_eq!(span, Span::from("Hello"));
    /// ```
    pub fn push_back(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_back(cell);
    }

    /// Pushes cell to front of span.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let mut span = Span::from("");
    ///
    /// span.push_front(Cell::new('o'));
    /// span.push_front(Cell::new('l'));
    /// span.push_front(Cell::new('l'));
    /// span.push_front(Cell::new('e'));
    /// span.push_front(Cell::new('H'));
    ///
    /// assert_eq!(span, Span::from("Hello"));
    /// ```
    pub fn push_front(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_front(cell);
    }

    /// Pops cell from back of span.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let mut span = Span::from("Hello, World!");
    ///
    /// for _ in 0..8 {
    ///     span.pop_back();
    /// }
    ///
    /// assert_eq!(span, Span::from("Hello"));
    /// ```
    pub fn pop_back(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_back()
    }

    /// Pops cell from front of span.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let mut span = Span::from("Hello, World!");
    ///
    /// for _ in 0..7 {
    ///     span.pop_front();
    /// }
    ///
    /// assert_eq!(span, Span::from("World!"));
    /// ```
    pub fn pop_front(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_front()
    }

    /// Truncates `num` cells to front.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let mut span = Span::from("Hi! Hello, World!");
    ///
    /// span.truncate_front(4);
    ///
    /// assert_eq!(span, Span::from("Hello, World!"));
    /// ```
    pub fn truncate_front(&mut self, num: u16) {
        let tmp = self.cells.drain(num as usize..).collect();

        self.len = self.len().max(num) - num;
        self.cells = tmp;
    }

    /// Truncates `num` cells to back.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let mut span = Span::from("Hello, World! yoo!");
    ///
    /// span.truncate_back(5);
    ///
    /// assert_eq!(span, Span::from("Hello, World!"));
    /// ```
    pub fn truncate_back(&mut self, num: u16) {
        self.len = self.len().max(num) - num;
        self.cells.truncate(self.len() as usize);
    }

    /// Append cells from other source.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let mut span = Span::from("Hello,");
    /// let mut source = Span::from(" World!");
    ///
    /// span.append(&mut source);
    ///
    /// assert_eq!(span, Span::from("Hello, World!"));
    /// ```
    pub fn append(&mut self, source: &mut Span) {
        self.len += source.len();

        for cell in source.cells.iter_mut() {
            self.cells.push_back(mem::take(cell));
        }
    }

    /// Pushes all from `s`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let mut span = Span::from("");
    ///
    /// span.push_all("Hello, ");
    /// span.push_all(Span::from("World!"));
    ///
    /// assert_eq!(span, Span::from("Hello, World!"));
    /// ```
    pub fn push_all<S: Into<Span>>(&mut self, s: S) {
        let mut span: Span = s.into();

        self.append(&mut span);
    }

    /// Returns copied span to [Vec].
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let span = Span::from("Hi!");
    ///
    /// let res = span.to_vec();
    ///
    /// assert_eq!(res[0], Cell::new('H'));
    /// assert_eq!(res[1], Cell::new('i'));
    /// assert_eq!(res[2], Cell::new('!'));
    /// ```
    pub fn to_vec(&self) -> Vec<Cell> {
        self.cells.iter().copied().collect()
    }

    /// Returns span parts by split indecies.
    ///
    /// `indecies` needs sorted.  
    /// Returned vec has `indecies` size +1 elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Span;
    /// let origin = Span::from("One, Two, Three");
    ///
    /// let spans = origin.split_by(&[5, 10, 99]);
    ///
    /// assert_eq!(spans[0], Some(Span::from("One, ")));
    /// assert_eq!(spans[1], Some(Span::from("Two, ")));
    /// assert_eq!(spans[2], Some(Span::from("Three")));
    /// assert_eq!(spans[3], None);
    /// ```
    pub fn split_by(&self, indecies: &[u16]) -> Vec<Option<Span>> {
        debug_assert!(indecies.is_sorted(), "indecies not sorted");

        let mut res = vec![];
        let mut i = 0usize;
        let cells = self.to_vec();

        for j in indecies {
            let j = *j as usize;

            match cells.get(i..j) {
                Some(cells) => {
                    res.push(Some(Span::from_iter(cells.iter().copied())));
                }
                None => {
                    res.push(cells.get(i..).map(|c| Span::from_iter(c.iter().copied())));
                    res.push(None);
                }
            }

            i = j;
        }

        if res.last().is_some() {
            res.push(cells.get(i..).map(|c| Span::from_iter(c.iter().copied())));
        }

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

impl From<Span> for Vec<Cell> {
    fn from(value: Span) -> Self {
        value.to_vec()
    }
}

impl Extend<Cell> for Span {
    fn extend<T: IntoIterator<Item = Cell>>(&mut self, iter: T) {
        let mut addr = 0;

        for cell in iter.into_iter() {
            self.cells.push_back(cell);
            addr += 1;
        }

        self.len += addr;
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
