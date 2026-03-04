use std::io::{self, Write};

use eired_display::{Annotate, VTerm};

use crate::config::RuntimeConfig;

pub trait Renderer<W: Write> {
    fn render(&mut self, config: &RuntimeConfig, cells: VTerm) -> io::Result<()>;

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

    pub(crate) fn create_diff(&self, new_term: &VTerm) -> Option<VTerm> {
        let Some(ref prev_cache) = self.prev_cache else {
            return Some(new_term.clone());
        };

        if prev_cache.len() != new_term.len() {
            return Some(new_term.clone());
        }

        let mut cells = vec![None; new_term.len()];
        let mut is_changed = false;

        for (i, new_cell) in new_term.iter().enumerate() {
            if prev_cache.get(i) != new_term.get(i) {
                cells[i] = *new_cell;

                if !is_changed {
                    is_changed = true;
                }
            }
        }

        is_changed.then_some(VTerm::new(new_term.width(), new_term.height(), cells))
    }
}

impl RenderOptimizer {
    pub(crate) fn new() -> Self {
        Self { prev_cache: None }
    }
}
