use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::{Annot, Cell, Layer, View};

#[derive(Default, PartialEq, Eq)]
/// A canvas of non merged layers.
pub struct Canvas {
    front: usize,
    width: u16,
    height: u16,
    layers: BTreeMap<usize, Annot<Layer>>,
}

impl Canvas {
    /// Get inner mapping ref.
    pub fn inner_vec(&self) -> Vec<(&usize, &Annot<Layer>)> {
        self.layers.iter().collect::<Vec<_>>()
    }

    fn apply_layer(&mut self, z_index: usize, layer: Annot<Layer>) {
        let (layer_offset_x, layer_offset_y) = layer.base_pos();

        self.width = self.width.max(layer_offset_x + layer.width());
        self.height = self.height.max(layer_offset_y + layer.height());

        self.layers.insert(z_index, layer);
        self.front = self.front.max(z_index + 1);
    }

    /// Overlaps `layer` to top.
    pub fn overlap_layer(&mut self, layer: Annot<Layer>) {
        self.apply_layer(self.front, layer);
    }

    /// Insert `layer` to `z_index`.
    pub fn insert(&mut self, z_index: usize, layer: Annot<Layer>) {
        self.apply_layer(z_index, layer);
    }

    /// Merge `layer` to `z_index`, or insert if not found `z_index` layer.
    pub fn merge(&mut self, z_index: usize, new_layer: Annot<Layer>) {
        let merged_layer = match self.layers.get(&z_index) {
            Some(layer) => layer.inner().overlap(layer.base_pos(), new_layer),
            None => new_layer,
        };

        self.apply_layer(z_index, merged_layer);
    }

    /// Insert `layer` to `z_index` if not found, or merge if found.
    pub fn insert_or_merge(&mut self, z_index: usize, layer: Annot<Layer>) {
        if self.layers.contains_key(&z_index) {
            self.merge(z_index, layer);
        } else {
            self.insert(z_index, layer);
        }
    }

    /// Create a [View] from `self`.
    pub fn create_view(&self) -> View {
        let mut view: Vec<Option<Cell>> = vec![None; self.height as usize * self.width as usize];

        for (_, layer) in self.layers.iter() {
            let (layer_offset_x, layer_offset_y) = layer.base_pos();
            let layer_spans = layer.inner().inner();

            for span in layer_spans.iter() {
                let (span_x, span_y) = span.base_pos();
                let (span_x, span_y) = (span_x + layer_offset_x, span_y + layer_offset_y);
                let line_pad = span_y * self.width;
                let replace_slice = &mut view
                    [(span_x + line_pad) as usize..(span_x + span.width() + line_pad) as usize];

                let cells = span
                    .inner()
                    .to_vec()
                    .into_iter()
                    .map(Option::Some)
                    .collect::<Vec<_>>();

                replace_slice.copy_from_slice(&cells);
            }
        }

        View::new(self.width, self.height, view)
    }
}

impl Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas")
            .field("front", &self.front)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("layers", &self.layers)
            .finish()
    }
}
