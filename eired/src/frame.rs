use std::mem;
use std::sync::Arc;

use crossbeam::channel::Sender;

use crate::{Annot, Canvas, Error, Point, RuntimeTask, Widget};

pub struct Frame {
    canvas: Canvas,
    size_fn: Arc<dyn Fn() -> (u16, u16)>,
    tx: Sender<RuntimeTask>,
    width: u16,
    height: u16,
    cursor: Option<Point>,
    cursor_vis: Option<bool>,
}

impl Frame {
    pub(crate) fn new(size: Arc<dyn Fn() -> (u16, u16)>, tx: Sender<RuntimeTask>) -> Self {
        let terminal_size = size();
        let width = terminal_size.0;
        let height = terminal_size.1;

        Self {
            canvas: Canvas::new(width, height),
            size_fn: size,
            tx,
            width,
            height,
            cursor: None,
            cursor_vis: None,
        }
    }

    fn take_canvas(&mut self) -> Canvas {
        self.on_resize();

        let new_canvas = Canvas::new(self.width, self.height);

        mem::replace(&mut self.canvas, new_canvas)
    }

    pub fn on_resize(&mut self) {
        let terminal_size = (self.size_fn)();

        if (self.width, self.height) != terminal_size {
            self.width = terminal_size.0;
            self.height = terminal_size.1;
            self.canvas = Canvas::new(self.width, self.height);
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cursor_move_to<P: Into<Point>>(&mut self, at: P) {
        self.cursor = Some(at.into());
    }

    pub fn show_cursor(&mut self) {
        self.cursor_vis = Some(true);
    }

    pub fn hide_cursor(&mut self) {
        self.cursor_vis = Some(false);
    }

    pub fn draw<T: Widget>(&mut self, cells: Annot<T>) {
        self.canvas.draw(cells);
    }

    pub fn update_frame(&mut self) -> crate::Result<()> {
        let canvas = self.take_canvas();

        if let Some(vis) = self.cursor_vis.take() {
            if vis {
                self.tx.send(RuntimeTask::ShowCursor).map_err(Error::Send)?;

                if let Some(cursor) = self.cursor.take() {
                    self.tx
                        .send(RuntimeTask::MoveCursor(cursor))
                        .map_err(Error::Send)?;
                }
            } else {
                self.tx.send(RuntimeTask::HideCursor).map_err(Error::Send)?;
                self.tx
                    .send(RuntimeTask::MoveCursor(Point::default()))
                    .map_err(Error::Send)?;
            }
        }

        self.tx
            .send(RuntimeTask::UpdateBuffer(canvas))
            .map_err(Error::Send)
    }
}
