use std::io::{self, Write};

use eired_display::{Annotate, VTerm};

pub trait Renderer<W: Write> {
    fn render(&mut self, cells: VTerm) -> io::Result<()>;
}

pub struct TerminalRenderer<W: Write> {
    base_pos: (u16, u16),
    writer: W,
}

impl<W: Write> TerminalRenderer<W> {
    pub fn new(base_pos: (u16, u16), writer: W) -> Self {
        Self { base_pos, writer }
    }
}

impl<W: Write> Renderer<W> for TerminalRenderer<W> {
    fn render(&mut self, cells: VTerm) -> io::Result<()> {
        let cmds = eired_display::convert_to_spans(cells.annotate(self.base_pos));

        for cmd in cmds {
            cmd.draw(&mut self.writer)?;
        }

        self.writer.flush()
    }
}

pub struct RenderOptimizer {
    prev_cache: VTerm,
}

impl RenderOptimizer {
    fn replace_cache(&mut self, new_cache: VTerm) {
        self.prev_cache = new_cache;
    }

    fn create_diff(&self, new_term: &VTerm) -> Option<VTerm> {
        if self.prev_cache.len() != new_term.len() {
            return Some(new_term.clone());
        }

        let mut cells = vec![None; new_term.len()];
        let mut is_changed = false;

        for (i, new_cell) in new_term.iter().enumerate() {
            if self.prev_cache.get(i) != new_term.get(i) {
                cells[i] = *new_cell;

                if !is_changed {
                    is_changed = true;
                }
            }
        }

        is_changed.then_some(VTerm::new(new_term.width(), new_term.height(), cells))
    }
}
