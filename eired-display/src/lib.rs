use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem;

use crossterm::style::Color;

pub struct Annot<T> {
    base_x: u16,
    base_y: u16,
    pub width: u16,
    pub height: u16,
    inner: T,
}

impl<T> Annot<T> {
    pub fn new(base: (u16, u16), width: u16, height: u16, inner: T) -> Self {
        Self {
            base_x: base.0,
            base_y: base.1,
            width,
            height,
            inner,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn base_pos(&self) -> (u16, u16) {
        (self.base_x, self.base_y)
    }

    pub fn inner_apex_pos(&self) -> (u16, u16) {
        (
            self.base_x + self.width.max(1) - 1,
            self.base_y + self.height.max(1) - 1,
        )
    }

    pub fn outer_apex_pos(&self) -> (u16, u16) {
        (self.base_x + self.width, self.base_y + self.height)
    }

    pub fn is_conflict<A>(&self, other: &Annot<A>) -> bool {
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

    pub fn contains_pos(&self, rel_x: u16, rel_y: u16) -> bool {
        let dummy: Annot<Option<Cell>> = Annot::new((rel_x, rel_y), 1, 1, None);

        self.is_conflict(&dummy)
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
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
            .field("width", &self.width)
            .field("height", &self.height)
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: PartialEq> Eq for Annot<T> {}

impl<T: PartialEq> PartialEq for Annot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base_x == other.base_x
            && self.base_y == other.base_y
            && self.width == other.width
            && self.height == other.height
            && self.inner == other.inner
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self::from(ch)
    }

    pub fn new_fg(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            ..Self::default()
        }
    }

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

#[derive(Default, PartialEq, Eq)]
pub struct Span {
    len: u16,
    cells: VecDeque<Cell>,
}

impl Span {
    pub fn new_with_bg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.bg = color);

        span
    }

    pub fn new_with_fg<S: AsRef<str>>(cells: S, color: Color) -> Self {
        let mut span = Span::from(cells.as_ref());

        span.cells.iter_mut().for_each(|cell| cell.fg = color);

        span
    }

    pub fn get(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    pub fn replace_at(&mut self, idx: usize, cell: Cell) -> Option<Cell> {
        self.get_mut(idx).map(|before| mem::replace(before, cell))
    }

    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_back(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_back(cell);
    }

    pub fn push_front(&mut self, cell: Cell) {
        self.len += 1;
        self.cells.push_front(cell);
    }

    pub fn pop_back(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<Cell> {
        if !self.is_empty() {
            self.len -= 1;
        }

        self.cells.pop_front()
    }

    pub fn truncate_front(&mut self, num: u16) {
        let tmp = self.cells.drain(num as usize..).collect();

        self.len = self.len().max(num) - num;
        self.cells = tmp;
    }

    pub fn truncate_back(&mut self, num: u16) {
        self.len = self.len().max(num) - num;
        self.cells.truncate(self.len() as usize);
    }

    pub fn to_vec(&self) -> Vec<Cell> {
        self.cells.iter().copied().collect()
    }

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

#[derive(Default, PartialEq, Eq)]
pub struct Layer {
    width: u16,
    height: u16,
    spans: Vec<Annot<Span>>,
}

impl Layer {
    pub fn inner(&self) -> &[Annot<Span>] {
        &self.spans
    }

    fn push_span(&mut self, span: Annot<Span>) {
        let end_pos = span.outer_apex_pos();

        self.width = self.width.max(end_pos.0);
        self.height = self.height.max(end_pos.1);

        self.spans.push(span);
    }

    fn resolve_conflict(span: &Annot<Span>, begin: u16, end: u16) -> Vec<Annot<Span>> {
        let is_include_begin = span.contains_pos(begin, 0);
        let is_include_end = span.contains_pos(end - 1, 0);
        let mut solved = vec![];

        match (is_include_begin, is_include_end) {
            (true, true) => {
                let mut parts = span.inner().split_by(&[begin, end]);

                debug_assert!(parts.len() == 3, "Span::split_by impl error");

                solved.extend([parts[0].take(), parts[2].take()]);
            }
            (true, false) => {
                let mut parts = span.inner().split_by(&[begin]);

                debug_assert!(parts.len() == 2, "Span::split_by impl error");

                solved.push(parts[0].take());
            }
            (false, true) => {
                let mut parts = span.inner().split_by(&[end]);

                debug_assert!(parts.len() == 2, "Span::split_by impl error");

                solved.push(parts[1].take());
            }
            (false, false) => {}
        }

        let (mut base_x, base_y) = span.base_pos();

        solved
            .into_iter()
            .filter_map(|e| {
                e.map(|elem| Annot::new((base_x, base_y), elem.len(), 1, elem))
                    .inspect(|elem| base_x += elem.width)
            })
            .collect()
    }

    pub fn push_span_write(&mut self, span: Annot<Span>) {
        if span.is_empty() {
            return;
        }

        let mut tmp = vec![];
        let (begin, end) = (span.base_pos().0, span.outer_apex_pos().0);

        while let Some(i_span) = self.spans.pop() {
            if !i_span.is_conflict(&span) {
                tmp.push(i_span);

                continue;
            }

            tmp.extend(Self::resolve_conflict(&i_span, begin, end));
        }

        for elem in tmp {
            if !elem.is_empty() {
                self.push_span(elem);
            }
        }

        debug_assert!(
            !self.spans.iter().any(|s| s.is_conflict(&span)),
            "Layer::push_span_write impl error"
        );

        self.push_span(span);
    }

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
                let (begin, end) = (i_span.base_pos().0, i_span.outer_apex_pos().0);

                tmp_deque.extend(Self::resolve_conflict(&tmp_elem, begin, end).into_iter());
            }
        }

        debug_assert!(!tmp_deque.is_empty(), "Layer::push_span_fixed impl error");

        self.spans.retain(|s| !s.is_empty());

        for elem in tmp {
            if !elem.is_empty() {
                self.push_span(elem);
            }
        }
    }

    pub fn push_span_only_valid(&mut self, span: Annot<Span>) {
        if span.is_empty() || self.spans.iter().any(|s| s.is_conflict(&span)) {
            return;
        }

        self.spans.retain(|s| !s.is_empty());
        self.push_span(span);
    }

    pub fn overlap(&self, upper: Layer) -> Layer {
        let mut new_layer = Layer::default();
        let init_spans = self.spans.to_vec();

        for i_span in init_spans {
            new_layer.push_span(i_span);
        }

        for overlap_span in upper.spans {
            new_layer.push_span_write(overlap_span);
        }

        new_layer
    }

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

#[derive(Default, PartialEq, Eq)]
pub struct Canvas {
    front: usize,
    width: u16,
    height: u16,
    layers: BTreeMap<usize, Layer>,
}

impl Canvas {
    fn apply_layer(&mut self, z_index: usize, layer: Layer) {
        self.width = self.width.max(layer.width);
        self.height = self.height.max(layer.height);

        self.layers.insert(z_index, layer);
        self.front = self.front.max(z_index + 1);
    }

    pub fn overlap_layer(&mut self, layer: Layer) {
        self.apply_layer(self.front, layer);
    }

    pub fn insert(&mut self, z_index: usize, layer: Layer) {
        self.apply_layer(z_index, layer);
    }

    pub fn merge(&mut self, offset: (u16, u16), z_index: usize, layer: Layer) {
        let mut new_layer = layer;

        new_layer.add_offset(offset);

        let merged_layer = match self.layers.get(&z_index) {
            Some(layer) => layer.overlap(new_layer),
            None => new_layer,
        };

        self.apply_layer(z_index, merged_layer);
    }

    pub fn insert_or_merge(&mut self, offset: (u16, u16), z_index: usize, layer: Layer) {
        if self.layers.contains_key(&z_index) {
            self.insert(z_index, layer);
        } else {
            self.merge(offset, z_index, layer);
        }
    }

    pub fn create_view(&self) -> View {
        let mut view: Vec<Option<Cell>> = vec![None; self.height as usize * self.width as usize];

        for (_, layer) in self.layers.iter() {
            for span in layer.inner().iter() {
                let (span_x, span_y) = span.base_pos();
                let cells = span.inner().to_vec();
                let Some(cells) = cells.get(0..span.width as usize) else {
                    continue;
                };

                for (i, cell) in cells.iter().enumerate() {
                    let index = (span_y * span.width + span_x) as usize + i;

                    view[index] = Some(*cell);
                }
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
pub struct View {
    width: u16,
    height: u16,
    cells: Vec<Option<Cell>>,
}

impl View {
    pub fn new(width: u16, height: u16, cells: Vec<Option<Cell>>) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn get_line(&self, rows: u16) -> &[Option<Cell>] {
        let start = (self.width * rows) as usize;
        let end = (self.width * (rows + 1)) as usize;

        &self.cells[start..end]
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
