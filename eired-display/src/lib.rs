mod annot;
mod canvas;
mod cell;
mod draw;
mod layer;
mod span;
mod view;
mod window;

pub use annot::{Annot, Annotate};
pub use canvas::Canvas;
pub use cell::Cell;
pub use draw::DrawableSpan;
pub use layer::Layer;
pub use span::Span;
pub use view::View;
pub use window::{VTerm, Window, convert_to_spans, create_virtual_terminal};

/// A marker struct that represents area.
pub struct Rect(pub u16, pub u16);

impl Rect {
    /// Create new rect.
    pub fn new(width: u16, height: u16) -> Self {
        Self(width, height)
    }
}

impl Annotate for Rect {
    fn get_size(&self) -> (u16, u16) {
        (self.0, self.1)
    }
}
