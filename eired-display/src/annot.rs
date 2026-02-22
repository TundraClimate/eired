use std::fmt::Debug;

use crate::Rect;

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

    /// Unwrap to inner.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Rebase annotated position.
    pub fn rebase<F: Fn(&mut u16, &mut u16)>(&mut self, f: F) {
        f(&mut self.base_x, &mut self.base_y);
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
        Self: Sized,
    {
        Annot::new(root, self)
    }

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
