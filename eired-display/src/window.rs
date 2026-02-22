use std::collections::VecDeque;
use std::mem;
use std::slice::Iter;

use crate::{Annot, Annotate, Cell, DrawableSpan, View};

/// A rect of used by actual rendering.
pub struct Window {
    width: u16,
    height: u16,
    views: VecDeque<Annot<View>>,
}

impl Window {
    /// Create new window.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            views: VecDeque::new(),
        }
    }

    /// Create new window with filled views.
    pub fn from_views(width: u16, height: u16, views: Vec<Annot<View>>) -> Self {
        Self {
            width,
            height,
            views: VecDeque::from_iter(views),
        }
    }

    /// Resize window.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Overlapping with `view`.
    pub fn overlap(&mut self, view: Annot<View>) {
        self.views.push_back(view);
    }
}

impl Annotate for Window {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

/// Convert to annotated [VTerm] from annotated [Window].
pub fn create_virtual_terminal(window: Annot<Window>) -> Annot<VTerm> {
    let root = window.base_pos();

    let mut window = window.into_inner();

    let window_width = window.width;
    let window_height = window.height;

    let mut holder = vec![None; (window_width * window_height) as usize];

    while let Some(view) = window.views.pop_front() {
        let (view_offset_x, view_offset_y) = view.base_pos();

        let drawable_width = window_width.min(view.width().saturating_sub(view_offset_x)) as usize;
        let drawable_height = window_height - view_offset_y;

        if drawable_width == 0 || drawable_height == 0 {
            continue;
        }

        let view = view.into_inner();

        for rel_y in 0..drawable_height {
            let line = &view.get_line(rel_y);
            let view_offset_x = view_offset_x as usize;

            let src = &line[view_offset_x..(view_offset_x + drawable_width)];
            let dst_begin = (window_width * (view_offset_y + rel_y)) as usize + view_offset_x;

            let dst = &mut holder[dst_begin..dst_begin + drawable_width];

            dst.copy_from_slice(src);
        }
    }

    VTerm::new(window_width, window_height, holder).annotate(root)
}

/// A wrapper of [`Vec<Option<Cell>>`].
pub struct VTerm {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl VTerm {
    /// Create new wrapper.
    pub fn new(width: u16, height: u16, cells: Vec<Option<Cell>>) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns inner length.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` was inner is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns an inner iter.
    pub fn iter<'a>(&'a self) -> Iter<'a, Option<Cell>> {
        self.cells.iter()
    }

    /// Unwraps self.
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

impl Annotate for VTerm {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

/// Convert to draw commands from [VTerm].
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
