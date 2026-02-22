use std::fmt::Debug;

use crate::Rect;

/// An annotation of coords for struct.
///
/// A simply struct, it only has `base-coords` and `inner`.
/// - `base-coords` is reperesents the position on a terminal.
///   The "position" is able to apply with whatever absolute and relative.
/// - `inner` is just a target of annotation.
///   Can takes `&` and `&mut` by `inner_*` function, or `into_inner()` can take an ownership.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use eired_display::Annot;
/// # use eired_display::Annotate;
/// # use eired_display::Rect;
/// fn use_annot(rect: Annot<Rect>) {
///     /// rect is inject a Rect(10, 10).
///     let (x, y) = rect.base_pos();
///
/// # }
/// # fn main() {
/// #   let rect = Rect(10, 10).annotate((0, 0));
/// #   let (x, y) = rect.base_pos();
///     let other = Rect(10, 10).annotate((0, 0));
///     let (ox, oy) = other.base_pos();
///
///     assert_eq!(x, ox);
///     assert_eq!(y, oy);
///     
///     assert_eq!(rect.inner(), other.inner());
/// }
/// ```
/// ---
/// Advanced usage:
///
/// See: [Annotate]
pub struct Annot<T> {
    base_x: u16,
    base_y: u16,
    inner: T,
}

impl<T> Annot<T> {
    /// Wrap struct with annot.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annot;
    /// # use eired_display::Rect;
    /// let annot = Annot::new((0, 0), Rect(1, 1));
    ///
    /// assert_eq!(annot.base_pos(), (0, 0));
    /// assert_eq!(annot.inner(), &Rect(1, 1));
    /// ```
    pub fn new(base: (u16, u16), inner: T) -> Self {
        Self {
            base_x: base.0,
            base_y: base.1,
            inner,
        }
    }

    /// Returns base position of annot.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annot;
    /// # use eired_display::Rect;
    /// let annot = Annot::new((3, 0), Rect(1, 1));
    ///
    /// assert_eq!(annot.base_pos(), (3, 0));
    /// assert_eq!(annot.inner(), &Rect(1, 1));
    /// ```
    pub fn base_pos(&self) -> (u16, u16) {
        (self.base_x, self.base_y)
    }

    /// Get inner ref.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annot;
    /// # use eired_display::Rect;
    /// let annot = Annot::new((0, 0), Rect(1, 1));
    ///
    /// assert_eq!(annot.inner(), &Rect(1, 1));
    /// ```
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Ger inner ref mut.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annot;
    /// # use eired_display::Rect;
    /// let mut annot = Annot::new((0, 0), Rect(1, 1));
    ///
    /// *annot.inner_mut() = Rect(10, 10);
    ///
    /// assert_eq!(annot.inner(), &Rect(10, 10));
    /// ```
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Unwrap to inner.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annot;
    /// # use eired_display::Rect;
    /// let annot = Annot::new((0, 0), Rect(1, 1));
    ///
    /// assert_eq!(annot.into_inner(), Rect(1, 1));
    /// // Compile error!
    /// // assert_eq!(annot.inner(), &Rect(1, 1));
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Rebase annotated position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annot;
    /// # use eired_display::Rect;
    /// let mut annot = Annot::new((1, 4), Rect(1, 1));
    ///
    /// annot.rebase(|x, y| {
    ///     *x += 3;
    ///     *y += 3;
    /// });
    ///
    /// assert_eq!(annot.base_pos(), (4, 7));
    /// ```
    pub fn rebase<F: Fn(&mut u16, &mut u16)>(&mut self, f: F) {
        f(&mut self.base_x, &mut self.base_y);
    }
}

impl<T: Annotate> Annot<T> {
    /// Returns inner width.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 2).annotate((0, 0));
    ///
    /// assert_eq!(rect.width(), 10);
    /// ```
    pub fn width(&self) -> u16 {
        self.inner().width()
    }

    /// Returns inner height.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 2).annotate((0, 0));
    ///
    /// assert_eq!(rect.height(), 2);
    /// ```
    pub fn height(&self) -> u16 {
        self.inner().height()
    }

    /// Returns `true` was inner is empty.
    ///
    /// Is "empty" if either `width` or `height` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 0).annotate((0, 0));
    ///
    /// assert!(rect.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.width() == 0 || self.inner.height() == 0
    }

    /// Returns lower bound apex position of annot.
    ///
    /// # Examples
    ///
    /// Basic:
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(15, 20).annotate((0, 0));
    ///
    /// assert_eq!(rect.inner_apex_pos(), (14, 19));
    /// ```
    ///
    /// With annot:
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 10).annotate((5, 2));
    ///
    /// assert_eq!(rect.inner_apex_pos(), (14, 11));
    /// ```
    pub fn inner_apex_pos(&self) -> (u16, u16) {
        (
            self.base_x + self.width().max(1) - 1,
            self.base_y + self.height().max(1) - 1,
        )
    }

    /// Returns upeer bound apex position of annot.
    ///
    /// # Examples
    ///
    /// Basic:
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(15, 20).annotate((0, 0));
    ///
    /// assert_eq!(rect.outer_apex_pos(), (15, 20));
    /// ```
    ///
    /// With annot:
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 10).annotate((5, 2));
    ///
    /// assert_eq!(rect.outer_apex_pos(), (15, 12));
    /// ```
    pub fn outer_apex_pos(&self) -> (u16, u16) {
        (self.base_x + self.width(), self.base_y + self.height())
    }

    /// Returns `true` with conflicts is `self` and `other`.
    ///
    /// # Examples
    ///
    /// Partial conflict:
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let self_rect = Rect(5, 5).annotate((0, 0));
    /// let other_rect = Rect(5, 5).annotate((4, 0));
    ///
    /// assert!(self_rect.is_conflict(&other_rect));
    ///
    /// let other_rect = Rect(5, 5).annotate((5, 0));
    ///
    /// assert!(!self_rect.is_conflict(&other_rect));
    /// ```
    ///
    /// Surround conflict:
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let self_rect = Rect(10, 10).annotate((0, 0));
    /// let other_rect = Rect(4, 4).annotate((3, 3));
    ///
    /// assert!(self_rect.is_conflict(&other_rect));
    /// assert!(other_rect.is_conflict(&self_rect));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 10).annotate((0, 0));
    ///
    /// assert!(rect.contains_pos(0, 0));
    /// assert!(rect.contains_pos(9, 0));
    /// assert!(rect.contains_pos(0, 9));
    /// assert!(rect.contains_pos(5, 5));
    /// assert!(!rect.contains_pos(10, 0));
    /// assert!(!rect.contains_pos(0, 10));
    /// assert!(!rect.contains_pos(10, 10));
    /// ```
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

/// A trait for applies with `width` and `height` to target.
///
/// This trait supplies `annotate((x, y))`, `get_size()`, `width()` and `height()`, but required
/// method is only the `get_size`.
/// `get_size` need returns `(width, height)` tuple for uses to `width` and `height` value returning.
///
/// # Examples
///
/// ```
/// # use eired_display::Annotate;
/// struct Rect {
///     width: u16,
///     height: u16,
/// }
///
/// impl Annotate for Rect {
///     fn get_size(&self) -> (u16, u16) {
///         (self.width, self.height)
///     }
/// }
///
/// # assert_eq!(Rect { width: 12, height: 8 }.annotate((0, 0)).inner().width, 12);
/// # assert_eq!(Rect { width: 12, height: 8 }.annotate((0, 0)).inner().height, 8);
/// ```
pub trait Annotate {
    /// Create new annot.
    ///
    /// Not recommended override this function.
    /// Default implement is equals to [`Annot::new`], generally thats so sufficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// # use eired_display::Annot;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 10).annotate((1, 2));
    ///
    /// assert_eq!(rect, Annot::new((1, 2), Rect(10, 10)));
    /// ```
    fn annotate(self, root: (u16, u16)) -> Annot<Self>
    where
        Self: Sized,
    {
        Annot::new(root, self)
    }

    /// Returns (`width`, `height`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 8);
    ///
    /// assert_eq!(rect.get_size(), (10, 8));
    /// ```
    fn get_size(&self) -> (u16, u16);

    /// Returns width.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 8);
    ///
    /// assert_eq!(rect.width(), 10);
    /// ```
    fn width(&self) -> u16 {
        self.get_size().0
    }

    /// Returns height.
    ///
    /// # Examples
    ///
    /// ```
    /// # use eired_display::Annotate;
    /// use eired_display::Rect;
    ///
    /// let rect = Rect(10, 8);
    ///
    /// assert_eq!(rect.height(), 8);
    /// ```
    fn height(&self) -> u16 {
        self.get_size().1
    }
}
