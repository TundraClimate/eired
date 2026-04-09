use std::fmt::Debug;

use crate::{Point, Rect};

pub struct Annot<T> {
    base: Point,
    inner: T,
}

impl<T> Annot<T> {
    pub fn new<P: Into<Point>>(root: P, inner: T) -> Self {
        Self {
            base: root.into(),
            inner,
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn rebase<F: Fn(&mut u16, &mut u16)>(&mut self, f: F) {
        f(&mut self.base.0, &mut self.base.1);
    }
}

impl<T: Annotate> Annot<T> {
    pub fn width(&self) -> u16 {
        self.inner.width()
    }

    pub fn height(&self) -> u16 {
        self.inner.height()
    }

    pub fn get_size(&self) -> (u16, u16) {
        self.inner.get_size()
    }

    pub fn has_zero(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    pub fn base(&self) -> Point {
        self.base
    }

    pub fn in_bound(&self) -> Point {
        Point::from((
            self.base.cols() + self.width().max(1) - 1,
            self.base.rows() + self.height().max(1) - 1,
        ))
    }

    pub fn out_bound(&self) -> Point {
        Point::from((
            self.base.cols() + self.width(),
            self.base.rows() + self.height(),
        ))
    }

    pub fn is_conflict<A: Annotate>(&self, other: &Annot<A>) -> bool {
        if self.has_zero() || other.has_zero() {
            return false;
        }

        let self_base = self.base();
        let self_out = self.out_bound();
        let other_base = other.base();
        let other_out = other.out_bound();

        self_out.cols() > other_base.cols()
            && other_out.cols() > self_base.cols()
            && self_out.rows() > other_base.rows()
            && other_out.rows() > self_base.rows()
    }

    pub fn contains<P: Into<Point>>(&self, p: P) -> bool {
        self.is_conflict(&Rect(1, 1).annotate(p))
    }
}

impl<T: Copy> Copy for Annot<T> {}

impl<T: Clone> Clone for Annot<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug> Debug for Annot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} on {:?}", self.inner(), self.base)
    }
}

impl<T: Eq> Eq for Annot<T> {}

impl<T: PartialEq> PartialEq for Annot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.inner == other.inner
    }
}

pub trait Annotate {
    fn annotate<P: Into<Point>>(self, root: P) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    fn get_size(&self) -> (u16, u16);

    fn width(&self) -> u16 {
        self.get_size().0
    }

    fn height(&self) -> u16 {
        self.get_size().1
    }
}
