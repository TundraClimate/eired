use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;
use std::slice::Iter;
use std::vec::IntoIter;

use crossterm::style::Color;

/// An annotation of coords for struct.
pub struct Annot<T> {
    base_x: u16,
    base_y: u16,
    inner: T,
}

impl<T> Annot<T> {
    /// Wrap struct with annot.
    pub fn new(base: (u16, u16), inner: T) -> Self {
        Self {
            base_x: base.0,
            base_y: base.1,
            inner,
        }
    }

    /// Returns base position of annot.
    pub fn base_pos(&self) -> (u16, u16) {
        (self.base_x, self.base_y)
    }

    /// Get inner ref.
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Ger inner ref mut.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Annotate> Annot<T> {
    /// Returns inner width.
    pub fn width(&self) -> u16 {
        self.inner().width()
    }

    /// Returns inner height.
    pub fn height(&self) -> u16 {
        self.inner().height()
    }

    /// Returns `true` was inner is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.width() == 0 || self.inner.height() == 0
    }

    /// Returns lower bound apex position of annot.
    pub fn inner_apex_pos(&self) -> (u16, u16) {
        (
            self.base_x + self.width().max(1) - 1,
            self.base_y + self.height().max(1) - 1,
        )
    }

    /// Returns upeer bound apex position of annot.
    pub fn outer_apex_pos(&self) -> (u16, u16) {
        (self.base_x + self.width(), self.base_y + self.height())
    }

    /// Returns `true` with conflicts is `self` and `other`.
    pub fn is_conflict<A: Annotate>(&self, other: &Annot<A>) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        let (self_base_x, self_base_y) = self.base_pos();
        let (other_base_x, other_base_y) = other.base_pos();
        let (self_outer_x, self_outer_y) = self.outer_apex_pos();
        let (other_outer_x, other_outer_y) = other.outer_apex_pos();

        self_outer_x > other_base_x
            && other_outer_x > self_base_x
            && self_outer_y > other_base_y
            && other_outer_y > self_base_y
    }

    /// Returns `true` with coords is contains annot area.
    pub fn contains_pos(&self, rel_x: u16, rel_y: u16) -> bool {
        let dummy: Annot<Rect> = Rect::new(1, 1).annotate((rel_x, rel_y));

        self.is_conflict(&dummy)
    }
}

impl<T: Copy> Copy for Annot<T> {}

impl<T: Clone> Clone for Annot<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            ..*self
        }
    }
}

impl<T: Debug> Debug for Annot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Annot")
            .field("(cols, rows)", &(self.base_x, self.base_y))
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: PartialEq> Eq for Annot<T> {}

impl<T: PartialEq> PartialEq for Annot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base_x == other.base_x && self.base_y == other.base_y && self.inner == other.inner
    }
}

pub trait Annotate {
    /// Create new annot.
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized;

    /// Returns (`width`, `height`).
    fn get_size(&self) -> (u16, u16);

    /// Returns width.
    fn width(&self) -> u16 {
        self.get_size().0
    }

    /// Returns height.
    fn height(&self) -> u16 {
        self.get_size().1
    }
}

/// A marker struct that represents area.
pub struct Rect(pub u16, pub u16);

impl Rect {
    /// Create new rect.
    pub fn new(width: u16, height: u16) -> Self {
        Self(width, height)
    }
}

impl Annotate for Rect {
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16) {
        (self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// A struct of corresponds terminal 1 pixel.
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    /// Create new cell.
    pub fn new(ch: char) -> Self {
        Self::from(ch)
    }

    /// Create new cell with foreground.
    pub fn new_fg(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            ..Self::default()
        }
    }

    /// Create new cell with background.
    pub fn new_bg(ch: char, bg: Color) -> Self {
        Self {
            ch,
            bg,
            ..Self::default()
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            ..Self::default()
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("ch", &self.ch)
            .field("fg", &self.fg)
            .field("bg", &self.bg)
            .finish()
    }
}

impl Annotate for Cell {
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16) {
        (1, 1)
    }
}

#[derive(Default, PartialEq, Eq)]
/// A list wrapper of lined cells.
pub struct Span {
    len: u16,
    cells: VecDeque<Cell>,
}

impl Span {
    /// Create new span with background.
    pub fn new_with_bg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.bg = color);

        span
    }

    /// Create new span with foreground.
    pub fn new_with_fg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.fg = color);

        span
    }

    /// Get 1 cell ref by `idx`.
    pub fn get(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    /// Get 1 cell ref mut by `idx`.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    /// Replaces at `idx` cell.
    pub fn replace_at(&mut self, idx: usize, cell: Cell) -> Option<Cell> {
        self.get_mut(idx).map(|before| mem::replace(before, cell))
    }

    /// Returns span length.
    pub fn len(&self) -> u16 {
        self.len
    }

    /// Returns `true` was length is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes cell to back of span.
    pub fn push_back(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_back(cell);
    }

    /// Pushes cell to front of span.
    pub fn push_front(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_front(cell);
    }

    /// Pops cell from back of span.
    pub fn pop_back(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_back()
    }

    /// Pops cell from front of span.
    pub fn pop_front(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_front()
    }

    /// Truncates `num` cells to front.
    pub fn truncate_front(&mut self, num: u16) {
        let tmp = self.cells.drain(num as usize..).collect();

        self.len = self.len().max(num) - num;
        self.cells = tmp;
    }

    /// Truncates `num` cells to back.
    pub fn truncate_back(&mut self, num: u16) {
        self.len = self.len().max(num) - num;
        self.cells.truncate(self.len() as usize);
    }

    /// Returns copied span to [Vec].
    pub fn to_vec(&self) -> Vec<Cell> {
        self.cells.iter().copied().collect()
    }

    /// Returns span parts by split indecies.
    pub fn split_by(&self, indecies: &[u16]) -> Vec<Option<Span>> {
        debug_assert!(indecies.is_sorted(), "indecies not sorted");

        let mut res = vec![];
        let mut i = 0usize;
        let cells = self.to_vec();

        for j in indecies {
            let j = *j as usize;

            res.push(cells.get(i..j).map(|c| Span::from_iter(c.iter().copied())));

            i = j;
        }

        res.push(cells.get(i..).map(|c| Span::from_iter(c.iter().copied())));

        res
    }
}

impl Clone for Span {
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            cells: self.cells.clone(),
        }
    }
}

impl From<&str> for Span {
    fn from(value: &str) -> Self {
        let mut span = Self::default();

        for c in value.chars() {
            span.push_back(Cell::new(c));
        }

        span
    }
}

impl From<String> for Span {
    fn from(value: String) -> Self {
        let mut span = Self::default();

        for c in value.chars() {
            span.push_back(Cell::new(c));
        }

        span
    }
}

impl FromIterator<Cell> for Span {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut span = Self::default();

        for c in iter.into_iter() {
            span.push_back(c);
        }

        span
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("len", &self.len)
            .field("cells", &self.cells.iter().enumerate().collect::<Vec<_>>())
            .finish()
    }
}

impl Annotate for Span {
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16) {
        (self.len(), 1)
    }
}

#[derive(Default, PartialEq, Eq)]
/// A layer of merged spans.
pub struct Layer {
    width: u16,
    height: u16,
    spans: Vec<Annot<Span>>,
}

impl Layer {
    /// Get inner slice.
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
        let is_include_begin = base.contains_pos(overlap_begin, 0);
        let is_include_end = base.contains_pos(overlap_end - 1, 0);
        let (base_x, base_y) = base.base_pos();
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

        solved.into_iter().flatten().collect()
    }

    /// Pushes span that overlap other spans.
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

    /// Pushes size fixed span by other spans.
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
    pub fn push_span_only_valid(&mut self, span: Annot<Span>) {
        if span.is_empty() || self.spans.iter().any(|s| s.is_conflict(&span)) {
            return;
        }

        self.spans.retain(|s| !s.is_empty());
        self.push_span(span);
    }

    /// Create overlaps another layer to `self`.
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
    pub fn add_offset(&mut self, offset: (u16, u16)) {
        self.width += offset.0;
        self.height += offset.1;

        for span in self.spans.iter_mut() {
            span.base_x += offset.0;
            span.base_y += offset.1;
        }
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
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

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

#[derive(PartialEq, Eq)]
/// An immutable list of cells for terminal area.
pub struct View {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl View {
    /// Create new struct.
    pub fn new(width: u16, height: u16, cells: Vec<Option<Cell>>) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns cell list length.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns `true` was inner is empty.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Get inner iter.
    pub fn iter<'a>(&'a self) -> Iter<'a, Option<Cell>> {
        self.cells.iter()
    }

    /// Get slice from `row`.
    pub fn get_line(&self, rows: u16) -> &[Option<Cell>] {
        if rows >= self.height {
            return &[];
        }

        let start = (self.width * rows) as usize;
        let end = (self.width * (rows + 1)) as usize;

        &self.cells[start..end]
    }
}

impl IntoIterator for View {
    type Item = Option<Cell>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<'a> IntoIterator for &'a View {
    type Item = &'a Option<Cell>;
    type IntoIter = Iter<'a, Option<Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

impl Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("View")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("cells", &self.cells)
            .finish()
    }
}

impl Annotate for View {
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

/// A rect of used by actual rendering.
pub struct Window {
    width: u16,
    height: u16,
    views: VecDeque<Annot<View>>,
}

impl Window {
    /// Create new window.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            views: VecDeque::new(),
        }
    }

    /// Resize window.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Overlapping with `view`.
    pub fn overlap(&mut self, view: Annot<View>) {
        self.views.push_back(view);
    }
}

impl Annotate for Window {
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

pub fn convert_to_buffer(_window: Annot<Window>) {
    todo!()
}
