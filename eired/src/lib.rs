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
