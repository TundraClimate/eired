use std::io::{self, Write};
use std::mem;

use eired_display::{Annot, Annotate, Cell, Span, VTerm};

use crate::config::RuntimeConfig;

#[derive(Clone)]
pub struct Diff {
    spans: Vec<Annot<Span>>,
}

impl Diff {
    pub fn new(spans: Vec<Annot<Span>>) -> Self {
        Self { spans }
    }

    pub fn into_vec(self) -> Vec<Annot<Span>> {
        self.spans
    }
}

impl From<&VTerm> for Diff {
    fn from(value: &VTerm) -> Self {
        let cells: Vec<Cell> = value.to_vec();

        let spans = cells
            .chunks(value.width().into())
            .map(|cs| Span::from_iter(cs.iter().copied()))
            .enumerate()
            .map(|(i, span)| span.annotate((i as u16, 0)))
            .collect();

        Self { spans }
    }
}

pub trait Renderer<W: Write> {
    fn render(&mut self, config: &RuntimeConfig, diff: Diff) -> io::Result<()>;

    fn store(&mut self, config: &RuntimeConfig) -> io::Result<()>;

    fn restore(&mut self, config: &RuntimeConfig) -> io::Result<()>;
}

pub(crate) struct RenderOptimizer {
    prev_cache: Option<VTerm>,
}

impl RenderOptimizer {
    pub(crate) fn replace_cache(&mut self, new_cache: Option<VTerm>) {
        self.prev_cache = new_cache;
    }

    pub(crate) fn create_diff(&self, new_term: &VTerm) -> Option<Diff> {
        let Some(ref prev_cache) = self.prev_cache else {
            return Some(Diff::from(new_term));
        };

        if prev_cache.len() != new_term.len() {
            return Some(Diff::from(new_term));
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

        (!spans.is_empty()).then_some(Diff::new(spans))
    }
}

impl RenderOptimizer {
    pub(crate) fn new() -> Self {
        Self { prev_cache: None }
    }
}
