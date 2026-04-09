use std::io::{self, Write};

use eired_display::VisualSpan;

use crate::Canvas;
use crate::config::RuntimeConfig;

pub trait Renderer<W: Write> {
    fn render(&mut self, config: &RuntimeConfig, canvas: &Canvas, diff: Diff) -> io::Result<()>;

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()>;

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()>;
}

pub(crate) struct RenderOptimizer {
    prev_canvas: Option<Canvas>,
    prev_cursor: Option<VisCursor>,
}

impl RenderOptimizer {
    pub(crate) fn new() -> Self {
        Self {
            prev_canvas: None,
            prev_cursor: None,
        }
    }

    pub(crate) fn cache(&mut self, new_cells: Option<Canvas>, new_cursor: Option<VisCursor>) {
        self.prev_canvas = new_cells;
        self.prev_cursor = new_cursor;
    }

    pub(crate) fn create_cursor_diff(
        &self,
        cursor: (u16, u16),
        cursor_vis: bool,
    ) -> Option<VisCursor> {
        let Some(ref prev_cache) = self.prev_cursor else {
            return Some(VisCursor { cursor, cursor_vis });
        };

        (prev_cache.cursor_vis != cursor_vis || prev_cache.cursor != cursor)
            .then_some(VisCursor { cursor, cursor_vis })
    }

    pub(crate) fn create_diff(
        &self,
        new_canvas: &Canvas,
        new_cursor: Option<VisCursor>,
    ) -> Option<Diff> {
        let Some(ref prev) = self.prev_canvas else {
            return Some(Diff::new(new_canvas, new_cursor));
        };

        if prev.inner.len() != new_canvas.inner.len() {
            return Some(Diff::new(new_canvas, new_cursor));
        }

        let prev = prev.inner.as_slice();
        let new = new_canvas.inner.as_slice();
        let width = new_canvas.width;

        let mut start;
        let mut i = 0;
        let mut ranges = vec![];

        while new.len() > i {
            if prev[i] == new[i] {
                i += 1;

                continue;
            }

            start = i;
            i += 1;

            while new.len() > i && !(i as u16).is_multiple_of(width) && prev[i] != new[i] {
                i += 1;
            }

            ranges.push((start, i));
        }

        (!ranges.is_empty() || new_cursor.is_some()).then_some(Diff {
            ranges,
            cursor: new_cursor,
        })
    }
}

pub struct Diff {
    pub(crate) ranges: Vec<(usize, usize)>,
    pub(crate) cursor: Option<VisCursor>,
}

impl Diff {
    fn new(canvas: &Canvas, cursor: Option<VisCursor>) -> Self {
        let mut ranges = vec![];
        let mut start = 0usize;

        for r in 1..=canvas.height {
            let length = canvas.width as usize * r as usize;

            ranges.push((start, length));
            start = length;
        }

        Self { ranges, cursor }
    }

    pub fn draws<'a>(self, base: (u16, u16), buffer: &'a Canvas) -> Vec<VisualSpan<'a>> {
        let base_cols = base.0;
        let base_rows = base.1;

        self.ranges
            .into_iter()
            .map(|range| {
                let cells = &buffer.inner.as_slice()[range.0..range.1];
                let cols = range.0 as u16 % buffer.width + base_cols;
                let rows = range.0 as u16 / buffer.width + base_rows;

                VisualSpan::new((cols, rows), cells)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VisCursor {
    pub(crate) cursor_vis: bool,
    pub(crate) cursor: (u16, u16),
}
