#![allow(dead_code)]

mod error;
mod frame;
mod tui;

pub mod style {
    pub use eired_display::{AnsiColor, Color, Style};
}

pub mod config {
    pub use eired_runtime::config::{ConfigBuilder, RuntimeConfig};
}

pub mod terminal {
    pub use eired_display::{Annot, Annotate, Cell, Point, Rect, VisualSpan};
    pub use eired_runtime::renderer::Renderer;
    pub use eired_runtime::terminal::TerminalRenderer;

    pub use eired_runtime::terminal::{cursor_move_to, cursor_pos, get_size};
}

pub mod runtime {
    pub use eired_runtime::RenderRuntime;
    pub use eired_runtime::task::RuntimeTask;

    pub use crate::frame::Frame;
}

pub mod widget;

pub use error::{Error, Result};
pub use tui::{TuiConfig, TuiEngine};
