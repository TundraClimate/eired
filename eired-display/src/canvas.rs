use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::{Annot, Cell, Layer, View};

#[derive(Default, PartialEq, Eq)]
/// A canvas of non merged layers.
///
/// Each layers in canvas has a `z_index`, keeps data and order as long as a `z_index` different else.
/// On [`create_view`](Canvas::create_view) calls, stacks each layer from lower `z_index` then
/// returns created view.
///
/// # Examples
///
/// ```
/// # use eired_display::Canvas;
/// use eired_display::Layer;
/// use eired_display::Annotate;
/// use eired_display::Span;
/// use eired_display::Cell;
///
/// let mut canvas = Canvas::default();
/// let mut layer = Layer::default();
///
/// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
/// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
/// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
///
/// canvas.overlap_layer(layer.annotate((0, 0)));
///
/// let mut layer = Layer::default();
///
/// layer.push_span_write(Span::from("OOO").annotate((0, 0)));
/// layer.push_span_write(Span::from("OOO").annotate((0, 1)));
///
/// canvas.overlap_layer(layer.annotate((2, 1)));
///
/// assert_eq!(canvas.inner_vec()[0].0, &0);
/// assert_eq!(canvas.inner_vec()[0].1.base_pos(), (0, 0));
/// assert_eq!(canvas.inner_vec()[1].0, &1);
/// assert_eq!(canvas.inner_vec()[1].1.base_pos(), (2, 1));
///
/// let view = canvas.create_view();
///
/// let mut view_iter = view.into_iter();
///
/// // Line 0
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(None));
/// assert_eq!(view_iter.next(), Some(None));
///
/// // Line 1
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
///
/// // Line 2
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
/// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
///
/// // End
/// assert_eq!(view_iter.next(), None);
/// ```
pub struct Canvas {
    front: usize,
    width: u16,
    height: u16,
    layers: BTreeMap<usize, Annot<Layer>>,
}

impl Canvas {
    /// Get inner mapping ref.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Canvas;
    /// let canvas = Canvas::default();
    ///
    /// assert!(canvas.inner_vec().is_empty());
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Canvas;
    /// use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut canvas = Canvas::default();
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// canvas.overlap_layer(layer.annotate((0, 0)));
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("OOO").annotate((0, 0)));
    /// layer.push_span_write(Span::from("OOO").annotate((0, 1)));
    ///
    /// canvas.overlap_layer(layer.annotate((2, 1)));
    ///
    /// assert_eq!(canvas.inner_vec()[0].0, &0);
    /// assert_eq!(canvas.inner_vec()[0].1.base_pos(), (0, 0));
    /// assert_eq!(canvas.inner_vec()[1].0, &1);
    /// assert_eq!(canvas.inner_vec()[1].1.base_pos(), (2, 1));
    /// ```
    pub fn overlap_layer(&mut self, layer: Annot<Layer>) {
        self.apply_layer(self.front, layer);
    }

    /// Insert `layer` to `z_index`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Canvas;
    /// use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut canvas = Canvas::default();
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// canvas.insert(5, layer.annotate((0, 0)));
    ///
    /// assert_eq!(canvas.inner_vec()[0].0, &5);
    /// assert_eq!(canvas.inner_vec()[0].1.base_pos(), (0, 0));
    /// ```
    pub fn insert(&mut self, z_index: usize, layer: Annot<Layer>) {
        self.apply_layer(z_index, layer);
    }

    /// Merge `layer` to `z_index` if found `z_index` layer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Canvas;
    /// use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut canvas = Canvas::default();
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// canvas.insert(0, layer.annotate((0, 0)));
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("OOO").annotate((0, 0)));
    /// layer.push_span_write(Span::from("OOO").annotate((0, 1)));
    ///
    /// canvas.merge(0, layer.annotate((2, 1)));
    ///
    /// assert_eq!(canvas.inner_vec().len(), 1);
    /// assert_eq!(canvas.inner_vec()[0].0, &0);
    /// assert_eq!(canvas.inner_vec()[0].1.width(), 5);
    /// assert_eq!(canvas.inner_vec()[0].1.height(), 3);
    /// ```
    pub fn merge(&mut self, z_index: usize, new_layer: Annot<Layer>) {
        let Some(merged_layer) = self.layers.get(&z_index) else {
            return;
        };

        let merged_layer = merged_layer
            .inner()
            .overlap(merged_layer.base_pos(), new_layer);

        self.apply_layer(z_index, merged_layer);
    }

    /// Insert `layer` to `z_index` if not found, or merge if found.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Canvas;
    /// use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut canvas = Canvas::default();
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// canvas.insert_or_merge(0, layer.annotate((0, 0)));
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("OOO").annotate((0, 0)));
    /// layer.push_span_write(Span::from("OOO").annotate((0, 1)));
    ///
    /// canvas.insert_or_merge(0, layer.annotate((2, 1)));
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("I").annotate((0, 0)));
    ///
    /// canvas.insert_or_merge(1, layer.annotate((3, 3)));
    ///
    /// assert_eq!(canvas.inner_vec().len(), 2);
    /// assert_eq!(canvas.inner_vec()[0].0, &0);
    /// assert_eq!(canvas.inner_vec()[0].1.width(), 5);
    /// assert_eq!(canvas.inner_vec()[0].1.height(), 3);
    /// assert_eq!(canvas.inner_vec()[1].0, &1);
    /// assert_eq!(canvas.inner_vec()[1].1.width(), 1);
    /// assert_eq!(canvas.inner_vec()[1].1.height(), 1);
    /// ```
    pub fn insert_or_merge(&mut self, z_index: usize, layer: Annot<Layer>) {
        if self.layers.contains_key(&z_index) {
            self.merge(z_index, layer);
        } else {
            self.insert(z_index, layer);
        }
    }

    /// Create a [View] from `self`.
    ///
    /// Stacks each layer from lower `z_index`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Canvas;
    /// use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    /// use eired_display::Cell;
    ///
    /// let mut canvas = Canvas::default();
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// layer.push_span_write(Span::from("O").annotate((1, 0)));
    /// layer.push_span_write(Span::from("OOO").annotate((0, 1)));
    /// layer.push_span_write(Span::from("O").annotate((1, 2)));
    ///
    /// layer.push_span_write(Span::from(" ").annotate((1, 1)));
    ///
    /// canvas.overlap_layer(layer.annotate((0, 0)));
    ///
    /// let view = canvas.create_view();
    ///
    /// let mut view_iter = view.into_iter();
    ///
    /// // Line 0
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
    ///
    /// // Line 1
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new(' '))));
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
    ///
    /// // Line 2
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('O'))));
    /// assert_eq!(view_iter.next(), Some(Some(Cell::new('X'))));
    ///
    /// // End
    /// assert_eq!(view_iter.next(), None);
    /// ```
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
