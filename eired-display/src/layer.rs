use std::collections::VecDeque;
use std::fmt::Debug;

use crate::{Annot, Annotate, Span};

#[derive(Default, PartialEq, Eq)]
/// A layer of merged spans.
///
/// Includes spans has annotated for hold coords of relative on terminal.
/// Spans only uses to push a [Canvas](crate::Canvas), may changes coords and size in always.
///
/// # Examples
///
/// ```
/// # use eired_display::Layer;
/// use eired_display::Annotate;
/// use eired_display::Span;
///
/// let mut layer = Layer::default();
///
/// // Square
/// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
/// layer.push_span_write(Span::from("XOX").annotate((0, 1)));
/// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
///
/// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
/// assert!(layer.take_with(|s| s.inner() == &Span::from("XOX")).is_some());
/// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
/// ```
pub struct Layer {
    width: u16,
    height: u16,
    spans: Vec<Annot<Span>>,
}

impl Layer {
    /// Get inner slice.
    ///
    /// Slice not sorted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XOX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// assert_eq!(layer.inner().len(), 3);
    /// ```
    pub fn inner(&self) -> &[Annot<Span>] {
        &self.spans
    }

    fn push_span(&mut self, span: Annot<Span>) {
        let end_pos = span.outer_apex_pos();

        self.width = self.width.max(end_pos.0);
        self.height = self.height.max(end_pos.1);

        self.spans.push(span);
    }

    fn resolve_conflict(base: Annot<Span>, overlap: &Annot<Span>) -> Vec<Annot<Span>> {
        if !base.is_conflict(overlap) {
            return vec![base];
        }

        let (overlap_begin, overlap_end) = (overlap.base_pos().0, overlap.outer_apex_pos().0);
        let (base_x, base_y) = base.base_pos();
        let is_include_begin = base.contains_pos(overlap_begin, base_y);
        let is_include_end = base.contains_pos(overlap_end - 1, base_y);
        let mut solved = vec![];

        match (is_include_begin, is_include_end) {
            (true, true) => {
                let (rel_begin, rel_end) = (overlap_begin - base_x, overlap_end - base_x);
                let mut parts = base.inner().split_by(&[rel_begin, rel_end]);

                debug_assert!(parts.len() == 3, "Span::split_by impl error");

                solved.extend([
                    parts[0].take().map(|p| p.annotate((base_x, base_y))),
                    parts[2].take().map(|p| p.annotate((overlap_end, base_y))),
                ]);
            }
            (true, false) => {
                let rel_begin = overlap_begin - base_x;
                let mut parts = base.inner().split_by(&[rel_begin]);

                debug_assert!(parts.len() == 2, "Span::split_by impl error");

                solved.push(parts[0].take().map(|p| p.annotate((base_x, base_y))));
            }
            (false, true) => {
                let rel_end = overlap_end - base_x;
                let mut parts = base.inner().split_by(&[rel_end]);

                debug_assert!(parts.len() == 2, "Span::split_by impl error");

                solved.push(parts[1].take().map(|p| p.annotate((overlap_end, base_y))));
            }
            (false, false) => {}
        }

        debug_assert!(!solved.is_empty(), "unsolved conflict");

        solved.into_iter().flatten().collect()
    }

    /// Pushes span that overlaps to above.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut layer = Layer::default();
    ///
    /// // Square
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// // Above square
    /// layer.push_span_write(Span::from("OOO").annotate((1, 1)));
    /// layer.push_span_write(Span::from("OOO").annotate((1, 2)));
    /// layer.push_span_write(Span::from("OOO").annotate((1, 3)));
    ///
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("X")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("X")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// ```
    pub fn push_span_write(&mut self, span: Annot<Span>) {
        if span.is_empty() {
            return;
        }

        let mut tmp = vec![];

        while let Some(i_span) = self.spans.pop() {
            tmp.extend(Self::resolve_conflict(i_span, &span));
        }

        for elem in tmp {
            if !elem.is_empty() {
                self.push_span(elem);
            }
        }

        debug_assert!(
            self.spans.iter().all(|s| !s.is_conflict(&span)),
            "Layer::push_span_write impl error"
        );

        self.push_span(span);
    }

    /// Pushes span that overlaps to below.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut layer = Layer::default();
    ///
    /// // Square
    /// layer.push_span_fixed(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_fixed(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_fixed(Span::from("XXX").annotate((0, 2)));
    ///
    /// // Above square
    /// layer.push_span_fixed(Span::from("OOO").annotate((1, 1)));
    /// layer.push_span_fixed(Span::from("OOO").annotate((1, 2)));
    /// layer.push_span_fixed(Span::from("OOO").annotate((1, 3)));
    ///
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("O")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("O")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// ```
    pub fn push_span_fixed(&mut self, span: Annot<Span>) {
        if span.is_empty() {
            return;
        }

        let mut tmp = vec![];
        let mut tmp_deque = VecDeque::new();
        let spans = self
            .spans
            .iter()
            .filter(|s| s.base_pos().1 == span.base_pos().1)
            .collect::<Vec<_>>();

        tmp_deque.push_back(span);

        loop {
            let Some(tmp_elem) = tmp_deque.pop_front() else {
                break;
            };

            let conflicts = spans
                .iter()
                .filter(|s| s.is_conflict(&tmp_elem))
                .collect::<Vec<_>>();

            if conflicts.is_empty() {
                tmp.push(tmp_elem);

                continue;
            }

            for i_span in conflicts.iter() {
                tmp_deque.extend(Self::resolve_conflict(tmp_elem.clone(), i_span).into_iter());
            }
        }

        debug_assert!(tmp_deque.is_empty(), "Layer::push_span_fixed impl error");

        self.spans.retain(|s| !s.is_empty());

        for elem in tmp {
            if !elem.is_empty() {
                self.push_span(elem);
            }
        }
    }

    /// Pushes span if not conflict other spans.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut layer = Layer::default();
    ///
    /// // Square
    /// layer.push_span_only_valid(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_only_valid(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_only_valid(Span::from("XXX").annotate((0, 2)));
    ///
    /// // Above square
    /// layer.push_span_only_valid(Span::from("OOO").annotate((1, 1)));
    /// layer.push_span_only_valid(Span::from("OOO").annotate((1, 2)));
    /// layer.push_span_only_valid(Span::from("OOO").annotate((1, 3)));
    ///
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// ```
    pub fn push_span_only_valid(&mut self, span: Annot<Span>) {
        if span.is_empty() || self.spans.iter().any(|s| s.is_conflict(&span)) {
            return;
        }

        self.spans.retain(|s| !s.is_empty());
        self.push_span(span);
    }

    /// Create overlapped another layer with `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut below = Layer::default();
    ///
    /// below.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// below.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// below.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// let mut above = Layer::default();
    ///
    /// above.push_span_write(Span::from("OOO").annotate((1, 1)));
    /// above.push_span_write(Span::from("OOO").annotate((1, 2)));
    /// above.push_span_write(Span::from("OOO").annotate((1, 3)));
    ///
    /// let mut layer = below.overlap((0, 0), above.annotate((0, 0)));
    ///
    /// assert!(layer.inner_mut().take_with(|s| s.inner() == &Span::from("XXX")).is_some());
    /// assert!(layer.inner_mut().take_with(|s| s.inner() == &Span::from("X")).is_some());
    /// assert!(layer.inner_mut().take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// assert!(layer.inner_mut().take_with(|s| s.inner() == &Span::from("X")).is_some());
    /// assert!(layer.inner_mut().take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// assert!(layer.inner_mut().take_with(|s| s.inner() == &Span::from("OOO")).is_some());
    /// ```
    pub fn overlap(&self, self_root: (u16, u16), upper: Annot<Layer>) -> Annot<Layer> {
        let mut new_layer = Layer::default();
        let init_spans = self.spans.to_vec();
        let (upper_x, upper_y) = upper.base_pos();

        for i_span in init_spans {
            new_layer.push_span(i_span);
        }

        for overlap_span in upper.into_inner().spans {
            let (rel_x, rel_y) = overlap_span.base_pos();
            let overlap_span = overlap_span
                .into_inner()
                .annotate((upper_x + rel_x, upper_y + rel_y));

            new_layer.push_span_write(overlap_span);
        }

        new_layer.annotate((self_root.0.min(upper_x), self_root.1.min(upper_y)))
    }

    /// Adds offset to left top.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("XXX").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    ///
    /// layer.add_offset((3, 3));
    ///
    /// assert!(layer.take_with(|s| s == &Span::from("XXX").annotate((3, 3))).is_some());
    /// assert!(layer.take_with(|s| s == &Span::from("XXX").annotate((3, 4))).is_some());
    /// assert!(layer.take_with(|s| s == &Span::from("XXX").annotate((3, 5))).is_some());
    /// ```
    pub fn add_offset(&mut self, offset: (u16, u16)) {
        self.width += offset.0;
        self.height += offset.1;

        for span in self.spans.iter_mut() {
            span.rebase(|x, y| {
                *x += offset.0;
                *y += offset.1;
            })
        }
    }

    /// Takes first element of matchs condition.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Layer;
    /// use eired_display::Annotate;
    /// use eired_display::Span;
    ///
    /// let mut layer = Layer::default();
    ///
    /// layer.push_span_write(Span::from("X").annotate((0, 0)));
    /// layer.push_span_write(Span::from("XX").annotate((0, 1)));
    /// layer.push_span_write(Span::from("XXX").annotate((0, 2)));
    /// layer.push_span_write(Span::from("OOOO").annotate((0, 3)));
    /// layer.push_span_write(Span::from("XXXXX").annotate((0, 4)));
    ///
    /// assert_eq!(layer.take_with(|s| s.width() == 4).unwrap().inner(), &Span::from("OOOO"));
    /// ```
    pub fn take_with<F: Fn(&Annot<Span>) -> bool>(&mut self, f: F) -> Option<Annot<Span>> {
        let pos = self.spans.iter().position(f);

        pos.map(|pos| self.spans.swap_remove(pos))
    }
}

impl Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("spans", &self.spans)
            .finish()
    }
}

impl Annotate for Layer {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
