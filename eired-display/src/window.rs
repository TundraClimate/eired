use std::collections::VecDeque;
use std::fmt::Debug;
use std::slice::Iter;
use std::vec::IntoIter;

use crate::{Annot, Annotate, Cell, Span, View};

#[derive(PartialEq, Eq)]
/// A rect of used by actual rendering.
///
/// This holds the layer overlaps on temp. NOT truncates and NOT overwrites. Window size can only
/// changes by [`resize`](Window::resize).
///
/// # Examples
///
/// ```
/// # use eired_display::Window;
/// use eired_display::Annotate;
/// use eired_display::Cell;
/// use eired_display::View;
///
/// let view = View::new(3, 1, vec![
///     Cell::new('O'),
///     Cell::new('O'),
///     Cell::new('O'),
/// ]);
///
/// let mut window = Window::new(5, 2);
///
/// window.overlap(view.clone().annotate((0, 0)));
/// window.overlap(view.clone().annotate((3, 1)));
///
/// assert_eq!(window, Window::from_views(5, 2, vec![
///     view.clone().annotate((0, 0)),
///     view.clone().annotate((3, 1)),
/// ]));
/// ```
pub struct Window {
    width: u16,
    height: u16,
    views: VecDeque<Annot<View>>,
}

impl Window {
    /// Create new window.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Window;
    /// use eired_display::Annotate;
    /// use eired_display::Cell;
    /// use eired_display::View;
    ///
    /// let view = View::new(3, 1, vec![
    ///     Cell::new('O'),
    ///     Cell::new('O'),
    ///     Cell::new('O'),
    /// ]);
    ///
    /// let mut window = Window::new(5, 2);
    ///
    /// window.overlap(view.clone().annotate((0, 0)));
    /// window.overlap(view.clone().annotate((3, 1)));
    ///
    /// assert_eq!(window, Window::from_views(5, 2, vec![
    ///     view.clone().annotate((0, 0)),
    ///     view.clone().annotate((3, 1)),
    /// ]));
    /// ```
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            views: VecDeque::new(),
        }
    }

    /// Create new window with filled views.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Window;
    /// use eired_display::Annotate;
    /// use eired_display::Cell;
    /// use eired_display::View;
    ///
    /// let window = Window::from_views(2, 2, vec![
    ///     View::new(3, 1, vec![Cell::default(), Cell::default(), Cell::default()]).annotate((0, 0)),
    ///     View::new(2, 1, vec![Cell::new('I'), Cell::default()]).annotate((0, 1)),
    /// ]);
    /// ```
    pub fn from_views(width: u16, height: u16, views: Vec<Annot<View>>) -> Self {
        Self {
            width,
            height,
            views: VecDeque::from_iter(views),
        }
    }

    /// Resize window.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Window;
    /// use eired_display::Annotate;
    /// use eired_display::Cell;
    /// use eired_display::View;
    ///
    /// let mut window = Window::from_views(2, 2, vec![
    ///     View::new(2, 1, vec![Cell::default(), Cell::default()]).annotate((0, 0)),
    ///     View::new(2, 1, vec![Cell::default(), Cell::default()]).annotate((0, 0)),
    /// ]);
    ///
    /// window.resize(3, 3);
    ///
    /// assert_eq!(window.width(), 3);
    /// assert_eq!(window.height(), 3);
    /// ```
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Overlapping with `view`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Window;
    /// use eired_display::Annotate;
    /// use eired_display::Cell;
    /// use eired_display::View;
    ///
    /// let view = View::new(3, 1, vec![
    ///     Cell::new('O'),
    ///     Cell::new('O'),
    ///     Cell::new('O'),
    /// ]);
    ///
    /// let mut window = Window::new(5, 2);
    ///
    /// window.overlap(view.clone().annotate((0, 0)));
    /// window.overlap(view.clone().annotate((3, 1)));
    ///
    /// assert_eq!(window, Window::from_views(5, 2, vec![
    ///     view.clone().annotate((0, 0)),
    ///     view.clone().annotate((3, 1)),
    /// ]));
    /// ```
    pub fn overlap(&mut self, view: Annot<View>) {
        self.views.push_back(view);
    }

    /// Convert to [VTerm] from [Window].
    ///
    /// [`VTerm`] inherit the size of [`Window`] and truncates the invisible sides.
    /// Write the layers in order, the last view displays on top.
    ///
    /// # Examples
    ///
    /// ```
    /// use eired_display::Window;
    /// use eired_display::Annotate;
    /// use eired_display::Cell;
    /// use eired_display::View;
    /// use eired_display::VTerm;
    ///
    /// let view = View::new(10, 1, vec![
    ///     Cell::new('I'),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::default(),
    ///     Cell::new('O'),
    /// ]);
    ///
    /// let window = Window::from_views(10, 5, vec![
    ///     view.clone().annotate((0, 0)),
    ///     view.clone().annotate((0, 1)),
    ///     view.clone().annotate((0, 2)),
    ///     view.clone().annotate((0, 3)),
    ///     view.clone().annotate((0, 4)),
    /// ]);
    ///
    /// let vterm = window.into_vterm();
    ///
    /// assert_eq!(vterm.len(), 50);
    /// ```
    pub fn into_vterm(mut self) -> VTerm {
        let window_width = self.width;
        let window_height = self.height;

        let mut holder = vec![Cell::default(); (window_width * window_height) as usize];

        while let Some(view) = self.views.pop_front() {
            let (view_margin_x, view_margin_y) = view.base_pos();

            let drawable_width = window_width
                .min(view.width() + view_margin_x)
                .saturating_sub(view_margin_x) as usize;
            let drawable_height = window_height
                .min(view.height() + view_margin_y)
                .saturating_sub(view_margin_y);

            if drawable_width == 0 || drawable_height == 0 {
                continue;
            }

            let view = view.into_inner();

            for rel_y in 0..drawable_height {
                let line = &view.get_line(rel_y);
                let view_margin_x = view_margin_x as usize;

                let src = &line[..drawable_width];
                let dst_begin = (window_width * (view_margin_y + rel_y)) as usize + view_margin_x;

                let dst = &mut holder[dst_begin..dst_begin + drawable_width];

                dst.copy_from_slice(src);
            }
        }

        VTerm::new(window_width, window_height, &holder)
    }
}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.views).finish()
    }
}

impl Annotate for Window {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

/// Convert to annotated [VTerm] from annotated [Window].
///
/// [`VTerm`] inherit the size of [`Window`] and truncates the invisible sides.
/// Write the layers in order, the last view displays on top.
///
/// # Examples
///
/// ```
/// use eired_display::Window;
/// use eired_display::Annotate;
/// use eired_display::Cell;
/// use eired_display::View;
/// use eired_display::VTerm;
///
/// let view = View::new(10, 1, vec![
///     Cell::new('I'),
///     Cell::default(),
///     Cell::default(),
///     Cell::default(),
///     Cell::default(),
///     Cell::default(),
///     Cell::default(),
///     Cell::default(),
///     Cell::default(),
///     Cell::new('O'),
/// ]);
///
/// let window = Window::from_views(10, 5, vec![
///     view.clone().annotate((0, 0)),
///     view.clone().annotate((0, 1)),
///     view.clone().annotate((0, 2)),
///     view.clone().annotate((0, 3)),
///     view.clone().annotate((0, 4)),
/// ]);
///
/// let vterm = eired_display::create_virtual_terminal(window.annotate((0, 0)));
///
/// assert_eq!(vterm.inner().len(), 50);
/// ```
pub fn create_virtual_terminal(window: Annot<Window>) -> Annot<VTerm> {
    let root = window.base_pos();

    window.into_inner().into_vterm().annotate(root)
}

#[derive(PartialEq, Eq, Clone)]
/// A wrapper of [`Vec<Cell>`].
///
/// # Examples
///
/// ```
/// # use eired_display::VTerm;
/// use eired_display::Span;
///
/// let vterm = VTerm::new(8, 2, &Span::from("1st line2nd line3rd line but ignore").to_vec());
///
/// assert_eq!(vterm.len(), 16);
/// assert_eq!(vterm.to_vec()[0..8], Span::from("1st line").to_vec());
/// assert_eq!(vterm.to_vec()[8..16], Span::from("2nd line").to_vec());
/// ```
pub struct VTerm {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

impl VTerm {
    /// Create new wrapper with `width * height` size.
    ///
    /// the cells of stuck out is truncates, the cells of not enough is fills with default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    ///
    /// let vterm = VTerm::new(10, 1, &Span::from("Hello, World!").to_vec());
    ///
    /// assert_eq!(vterm.len(), 10);
    /// assert_eq!(vterm.to_vec(), Span::from("Hello, Wor").to_vec());
    /// ```
    pub fn new(width: u16, height: u16, cells: &[Cell]) -> Self {
        let mut cells = cells.to_vec();

        if cells.len() > (width * height) as usize {
            cells.truncate((width * height) as usize);
        } else {
            cells.resize((width * height) as usize, Cell::default());
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Create new wrapper with `width * height` size.
    ///
    /// the cells of stuck out is truncates, the cells of not enough is fills with default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    ///
    /// let vterm = VTerm::from_lines(10, 3, &[
    ///     Span::from("=========="),
    ///     Span::from("Hi, World!"),
    ///     Span::from("=========="),
    ///     Span::from("This line is not includes"),
    /// ]);
    ///
    /// assert_eq!(vterm.len(), 30);
    /// assert_eq!(&vterm.to_vec()[0..10], &Span::from("==========").to_vec());
    /// assert_eq!(&vterm.to_vec()[10..20], &Span::from("Hi, World!").to_vec());
    /// assert_eq!(&vterm.to_vec()[20..30], &Span::from("==========").to_vec());
    /// ```
    pub fn from_lines(width: u16, height: u16, lines: &[Span]) -> Self {
        let xw = width as usize;
        let yh = height as usize;
        let mut cells = vec![Cell::default(); xw * yh];

        for (i, line) in lines.iter().enumerate() {
            if i >= yh {
                break;
            }

            let mut line = line.to_vec();

            line.truncate(xw);

            let pad = i * xw;
            let dst = &mut cells[pad..(pad + line.len())];
            let src = line.as_mut_slice();

            dst.swap_with_slice(src);
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns inner length.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    ///
    /// let vterm = VTerm::new(3, 1, &Span::from("IoI").to_vec());
    ///
    /// assert_eq!(vterm.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` was inner is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// let vterm = VTerm::new(0, 1, &[]);
    ///
    /// assert!(vterm.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns cell reference at `idx`.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(6, 1, &Span::from("Ilegal").to_vec());
    ///
    /// assert_eq!(vterm.get(0), Some(&Cell::new('I')));
    /// ```
    pub fn get(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    /// Returns an inner iter.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(4, 1, &Span::from("ItoO").to_vec());
    ///
    /// let mut iter = vterm.iter();
    ///
    /// assert_eq!(iter.next(), Some(&Cell::new('I')));
    /// assert_eq!(iter.next(), Some(&Cell::new('t')));
    /// assert_eq!(iter.next(), Some(&Cell::new('o')));
    /// assert_eq!(iter.next(), Some(&Cell::new('O')));
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, Cell> {
        self.cells.iter()
    }

    /// Unwraps self.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(3, 1, &Span::from("Vec").to_vec());
    ///
    /// let v = vterm.to_vec();
    ///
    /// assert_eq!(v, vec![
    ///     Cell::new('V'),
    ///     Cell::new('e'),
    ///     Cell::new('c'),
    /// ]);
    /// ```
    pub fn to_vec(&self) -> Vec<Cell> {
        self.cells.to_vec()
    }

    /// Returns inner value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(3, 1, &Span::from("Vec").to_vec());
    ///
    /// let v = vterm.into_vec();
    ///
    /// assert_eq!(v, vec![
    ///     Cell::new('V'),
    ///     Cell::new('e'),
    ///     Cell::new('c'),
    /// ]);
    /// ```
    pub fn into_vec(self) -> Vec<Cell> {
        self.cells
    }
}

impl IntoIterator for VTerm {
    type Item = Cell;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<'a> IntoIterator for &'a VTerm {
    type Item = &'a Cell;
    type IntoIter = Iter<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Debug for VTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.cells).finish()
    }
}

impl Annotate for VTerm {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
