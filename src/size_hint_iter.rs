#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Used to override the output of `Iterator::size_hint` of an iterator
pub struct SizeHintedIter<I> {
    inner: I,
    /// The lower bound to pretend to have
    pub lower: usize,
    /// The upper bound to pretend to have
    pub upper: Option<usize>,
}

impl<I: Iterator> SizeHintedIter<I> {
    #[inline(always)]
    /// Sets the bounds from the given iterator's `Iterator::size_hint`.
    pub fn new(inner: I) -> Self {
        let (lower, upper) = inner.size_hint();
        Self::with_bounds(inner, lower, upper)
    }
}
impl<I> SizeHintedIter<I> {
    #[inline(always)]
    /// Sets the bounds to the given arguments
    pub fn with_bounds(inner: I, lower: usize, upper: Option<usize>) -> Self {
        SizeHintedIter {
            inner,
            lower,
            upper,
        }
    }
    #[inline(always)]
    /// Turns `self` into the inner iterator
    pub fn into_inner(self) -> I {
        self.inner
    }
}

impl<I: Iterator> Iterator for SizeHintedIter<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.lower = self.lower.saturating_sub(1);
        if let Some(ref mut upper) = self.upper {
            *upper = upper.saturating_sub(1);
        }
        self.inner.next()
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.lower, self.upper)
    }
}
