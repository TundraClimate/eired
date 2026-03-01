use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;
use std::slice::Iter;

use crate::{Annot, Annotate, Cell, DrawableSpan, View};

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
///     Some(Cell::new('O')),
///     Some(Cell::new('O')),
///     Some(Cell::new('O')),
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
    ///     Some(Cell::new('O')),
    ///     Some(Cell::new('O')),
    ///     Some(Cell::new('O')),
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
    ///     View::new(3, 1, vec![None, None, None]).annotate((0, 0)),
    ///     View::new(2, 1, vec![Some(Cell::new('I')), None]).annotate((0, 1)),
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
    ///     View::new(2, 1, vec![None, None]).annotate((0, 0)),
    ///     View::new(2, 1, vec![None, None]).annotate((0, 0)),
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
    ///     Some(Cell::new('O')),
    ///     Some(Cell::new('O')),
    ///     Some(Cell::new('O')),
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
///     Some(Cell::new('I')),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     Some(Cell::new('O')),
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

    let mut window = window.into_inner();

    let window_width = window.width;
    let window_height = window.height;

    let mut holder = vec![None; (window_width * window_height) as usize];

    while let Some(view) = window.views.pop_front() {
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

    VTerm::new(window_width, window_height, holder).annotate(root)
}

#[derive(PartialEq, Eq)]
/// A wrapper of [`Vec<Option<Cell>>`].
///
/// # Examples
///
/// ```
/// # use eired_display::VTerm;
/// # use eired_display::View;
/// # use eired_display::Cell;
/// # use eired_display::Annotate;
/// # use eired_display::Window;
/// # let view = View::new(10, 1, vec![
/// #     Some(Cell::new('I')),
/// #     None,
/// #     None,
/// #     None,
/// #     None,
/// #     None,
/// #     None,
/// #     None,
/// #     None,
/// #     Some(Cell::new('O')),
/// # ]);
/// # let window = Window::from_views(10, 5, vec![
/// #     view.clone().annotate((0, 0)),
/// #     view.clone().annotate((0, 1)),
/// #     view.clone().annotate((0, 2)),
/// #     view.clone().annotate((0, 3)),
/// #     view.clone().annotate((0, 4)),
/// # ]);
/// let vterm = eired_display::create_virtual_terminal(
///     // Window
///     # window.annotate((0, 0))
/// );
///
/// assert_eq!(
///     vterm,
///     // term
///     # VTerm::new(10, 5, vec![
///     # Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
///     # Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
///     # Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
///     # Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
///     # Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
///     # ]).annotate((0, 0))
/// )
/// ```
pub struct VTerm {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl VTerm {
    /// Create new wrapper.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(10, 5, vec![
    ///     Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
    ///     Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
    ///     Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
    ///     Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
    ///     Some(Cell::new('I')), None, None, None, None, None, None, None, None, Some(Cell::new('O')),
    /// ]);
    ///
    /// assert_eq!(vterm.len(), 50);
    /// ```
    pub fn new(width: u16, height: u16, cells: Vec<Option<Cell>>) -> Self {
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
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(3, 1, vec![
    ///     Some(Cell::new('I')), None, Some(Cell::new('O')),
    /// ]);
    ///
    /// assert_eq!(vterm.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` was inner is empty.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// let vterm = VTerm::new(0, 1, vec![
    /// ]);
    ///
    /// assert!(vterm.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns an inner iter.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(3, 1, vec![
    ///     Some(Cell::new('I')), None, Some(Cell::new('O')),
    /// ]);
    ///
    /// let mut iter = vterm.iter();
    ///
    /// assert_eq!(iter.next(), Some(&Some(Cell::new('I'))));
    /// assert_eq!(iter.next(), Some(&None));
    /// assert_eq!(iter.next(), Some(&Some(Cell::new('O'))));
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, Option<Cell>> {
        self.cells.iter()
    }

    /// Unwraps self.
    ///
    /// ```
    /// # use eired_display::VTerm;
    /// use eired_display::Cell;
    ///
    /// let vterm = VTerm::new(3, 1, vec![
    ///     Some(Cell::new('I')), None, Some(Cell::new('O')),
    /// ]);
    ///
    /// let v = vterm.to_vec();
    ///
    /// assert_eq!(v, vec![
    ///     Some(Cell::new('I')),
    ///     None,
    ///     Some(Cell::new('O')),
    /// ]);
    /// ```
    pub fn to_vec(&self) -> Vec<Option<Cell>> {
        self.cells.to_vec()
    }
}

impl<'a> IntoIterator for &'a VTerm {
    type Item = &'a Option<Cell>;
    type IntoIter = Iter<'a, Option<Cell>>;

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

/// Convert to draw commands from [VTerm].
///
/// ```
/// # use eired_display::VTerm;
/// use eired_display::Cell;
/// use eired_display::Annotate;
///
/// let vterm = VTerm::new(5, 4, vec![
///     Some(Cell::new('S')), Some(Cell::new('P')), Some(Cell::new('A')), Some(Cell::new('N')), None,
///     None, Some(Cell::new('S')), Some(Cell::new('P')), Some(Cell::new('A')), Some(Cell::new('N')),
///     None, None, Some(Cell::new('S')), Some(Cell::new('P')), Some(Cell::new('A')),
///     Some(Cell::new('N')), None, None, Some(Cell::new('S')), Some(Cell::new('P')),
/// ]);
///
/// let spans = eired_display::convert_to_spans(vterm.annotate((0, 0)));
///
/// // "SPAN "
/// // " SPAN"
/// // "  SPA"
/// // "N  SP"
/// //
/// // INTO
/// //
/// // MoveTo(0, 0) "SPAN"
/// // MoveTo(1, 1) "SPAN"
/// // MoveTo(2, 2) "SPA"
/// // MoveTo(0, 3) "N"
/// // MoveTo(3, 3) "SP"
/// assert_eq!(spans.len(), 5);
/// ```
pub fn convert_to_spans(vterm: Annot<VTerm>) -> Vec<DrawableSpan> {
    let (rel_base_x, rel_base_y) = vterm.base_pos();
    let term_width = vterm.width();
    let vterm = vterm.into_inner();

    let mut res = vec![];
    let mut buffer = vec![];
    let mut start_x = rel_base_x;
    let mut start_y = rel_base_y;

    for (i, cell) in vterm.iter().enumerate() {
        if (i as u16).is_multiple_of(term_width) && !buffer.is_empty() {
            let cmd = DrawableSpan::new((start_x, start_y), mem::take(&mut buffer));

            res.push(cmd);
        }

        match cell {
            Some(cell) => {
                if buffer.is_empty() {
                    start_x = rel_base_x + (i as u16 % term_width);
                    start_y = rel_base_y + (i as u16 / term_width);
                }

                buffer.push(*cell);
            }
            None => {
                if !buffer.is_empty() {
                    let cmd = DrawableSpan::new((start_x, start_y), mem::take(&mut buffer));

                    res.push(cmd);
                }
            }
        }
    }

    if !buffer.is_empty() {
        let cmd = DrawableSpan::new((start_x, start_y), mem::take(&mut buffer));

        res.push(cmd);
    }

    res
}
