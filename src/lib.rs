#![warn(missing_docs, trivial_casts, trivial_numeric_casts)]
//! Crate for collecting an iterator of results into a result of a collection

use std::iter::{
    FromIterator,
    Extend,
    once,
    empty
};

mod size_hint_iter;
pub use size_hint_iter::SizeHintedIter;

/// Short circuiting collect of an iterator
pub trait CollectResult<T, E> {
    /// Collects into a result from the iterator
    fn collect_result<O: FromIterator<T> + Extend<T>>(self) -> Result<O, E>;
}

impl<T, E, I: Iterator<Item=Result<T, E>>> CollectResult<T, E> for I {
    /// Takes an iterator of results and tries to collect all `Ok`s from it
    ///
    /// Returns the Err if an item is ever `Err` short-circuting.
    /// That is, it will immediately return the error when it encounters the first one.
    fn collect_result<O: FromIterator<T> + Extend<T>>(self) -> Result<O, E> {
        let (mut lower, mut upper) = self.size_hint();
        let mut ret: O = SizeHintedIter::with_bounds(empty(), lower, upper).collect();

        for element in self {
            let iter = SizeHintedIter::with_bounds(once(element?), lower, upper);
            ret.extend(iter);

            lower = lower.saturating_sub(1);
            if let Some(ref mut upper) = upper {
                *upper = upper.saturating_sub(1);
            }
        }

        Ok(ret)
    }
}
