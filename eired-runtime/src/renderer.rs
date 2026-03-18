use std::io::{self, Write};
use std::mem;

use eired_display::{Annot, Annotate, Span, VTerm};

use crate::config::RuntimeConfig;

#[derive(Clone, Debug)]
pub struct Diff {
    spans: Vec<Annot<Span>>,
    cursor: Option<VisCursor>,
}

impl Diff {
    pub fn new(spans: Vec<Annot<Span>>, cursor: Option<VisCursor>) -> Self {
        Self { spans, cursor }
    }

    pub fn into_vec(self) -> Vec<Annot<Span>> {
        self.spans
    }

    pub fn cursor(&self) -> Option<VisCursor> {
        self.cursor
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VisCursor {
    cursor: (u16, u16),
    cursor_vis: bool,
}

impl VisCursor {
    fn new(cursor: (u16, u16), cursor_vis: bool) -> Self {
        Self { cursor, cursor_vis }
    }

    pub fn get_at(&self) -> (u16, u16) {
        self.cursor
    }

    pub fn is_visible(&self) -> bool {
        self.cursor_vis
    }
}

impl TryFrom<(&VTerm, Option<VisCursor>)> for Diff {
    type Error = &'static str;

    fn try_from(value: (&VTerm, Option<VisCursor>)) -> Result<Self, Self::Error> {
        let cursor = value.1;
        let value = value.0;
        let cells = value.to_vec();

        if cells.is_empty() {
            return Err("value size is zero");
        }

        if cells.len() != (value.width() * value.height()).into() {
            return Err("value size is incorrect");
        }

        let spans = cells
            .chunks(value.width().into())
            .map(|cs| Span::from_iter(cs.iter().copied()))
            .enumerate()
            .map(|(i, span)| span.annotate((0, i as u16)))
            .collect();

        Ok(Self { spans, cursor })
    }
}

pub trait Renderer<W: Write> {
    fn render(&mut self, config: &RuntimeConfig, diff: Diff) -> io::Result<()>;

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()>;

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()>;
}

pub(crate) struct RenderOptimizer {
    prev_cells: Option<VTerm>,
    prev_cursor: Option<VisCursor>,
}

impl RenderOptimizer {
    pub(crate) fn replace_cache(
        &mut self,
        new_cells: Option<VTerm>,
        new_cursor: Option<VisCursor>,
    ) {
        self.prev_cells = new_cells;
        self.prev_cursor = new_cursor;
    }

    pub(crate) fn create_cursor_diff(
        &self,
        cursor: (u16, u16),
        cursor_vis: bool,
    ) -> Option<VisCursor> {
        let Some(prev_cache) = self.prev_cursor else {
            return Some(VisCursor::new(cursor, cursor_vis));
        };

        (prev_cache.cursor_vis != cursor_vis || prev_cache.cursor != cursor)
            .then_some(VisCursor::new(cursor, cursor_vis))
    }

    pub(crate) fn create_diff(
        &self,
        new_term: &VTerm,
        new_cursor: Option<VisCursor>,
    ) -> Option<Diff> {
        let Some(ref prev_cache) = self.prev_cells else {
            return Diff::try_from((new_term, new_cursor)).ok();
        };

        if prev_cache.len() != new_term.len() {
            return Diff::try_from((new_term, new_cursor)).ok();
        }

        let cells = prev_cache.iter().zip(new_term.iter()).collect::<Vec<_>>();
        let mut spans = vec![];
        let mut buffer = vec![];

        for (y, line) in cells.chunks(new_term.width().into()).enumerate() {
            let y = y as u16;
            let mut sx = None;

            for (x, (prev, new)) in line.iter().enumerate() {
                let prev = **prev;
                let new = **new;

                if prev != new {
                    if sx.is_none() {
                        sx = Some(x as u16);
                    }

                    buffer.push(new);
                } else if let Some(sx) = sx.take() {
                    spans.push(Span::from_iter(mem::take(&mut buffer)).annotate((sx, y)))
                }
            }

            if let Some(sx) = sx.take() {
                spans.push(Span::from_iter(mem::take(&mut buffer)).annotate((sx, y)));
            }
        }

        (!spans.is_empty()).then_some(Diff::new(spans, new_cursor))
    }
}

impl RenderOptimizer {
    pub(crate) fn new() -> Self {
        Self {
            prev_cells: None,
            prev_cursor: None,
        }
    }
}
