use std::fmt::Debug;

use crossterm::style::Color;

use crate::Annotate;

#[derive(Clone, Copy, PartialEq, Eq)]
/// A struct that corresponds terminal 1 pixel.
///
/// Includes character, foreground color, background color.
///
/// # Note
///
/// `color` is the [`crossterm::style::Color`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html).  
/// This may change in the future.
///
/// # Examples
///
/// ```
/// # use eired_display::Cell;
/// use crossterm::style::Color;
///
/// let cell = Cell::new('A');
///
/// assert_eq!(cell, Cell { ch: 'A', fg: Color::Reset, bg: Color::Reset });
/// ```
pub struct Cell {
    /// A character that corresponds terminal pixel.
    ///
    /// Not ascii char: needs 2~ pixel but cell can includes all char.
    pub ch: char,

    /// A foreground color of pixel.
    ///
    /// # Note
    ///
    /// `color` is the [`crossterm::style::Color`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html).  
    /// This may change in the future.
    pub fg: Color,

    /// A background color of pixel.
    ///
    /// # Note
    ///
    /// `color` is the [`crossterm::style::Color`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html).  
    /// This may change in the future.
    pub bg: Color,
}

impl Cell {
    /// Create new cell.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Cell;
    /// use crossterm::style::Color;
    ///
    /// let cell = Cell::new('A');
    ///
    /// assert_eq!(cell, Cell { ch: 'A', fg: Color::Reset, bg: Color::Reset });
    /// ```
    pub fn new(ch: char) -> Self {
        Self::from(ch)
    }

    /// Create new cell with foreground.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Cell;
    /// use crossterm::style::Color;
    ///
    /// let cell = Cell::new_fg('A', Color::Red);
    ///
    /// assert_eq!(cell, Cell { ch: 'A', fg: Color::Red, bg: Color::Reset });
    /// ```
    pub fn new_fg(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            ..Self::default()
        }
    }

    /// Create new cell with background.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Cell;
    /// use crossterm::style::Color;
    ///
    /// let cell = Cell::new_bg('B', Color::Blue);
    ///
    /// assert_eq!(cell, Cell { ch: 'B', fg: Color::Reset, bg: Color::Blue });
    /// ```
    pub fn new_bg(ch: char, bg: Color) -> Self {
        Self {
            ch,
            bg,
            ..Self::default()
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            ..Self::default()
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("ch", &self.ch)
            .field("fg", &self.fg)
            .field("bg", &self.bg)
            .finish()
    }
}

impl Annotate for Cell {
    fn get_size(&self) -> (u16, u16) {
        (1, 1)
    }
}
