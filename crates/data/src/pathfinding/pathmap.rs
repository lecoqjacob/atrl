use ordered_float::OrderedFloat;

use crate::prelude::*;

pub trait PathMap {
    type ExitIterator: Iterator<Item = (IVec2, OrderedFloat<f32>)>;

    /// Returns an iterator of the valid list of successors for a given node
    fn successors(&self, p: &impl Point2d) -> Self::ExitIterator;

    /// The cost of moving to a given node
    fn cost(&self, a: impl Point2d) -> OrderedFloat<f32>;

    /// The distance between two node points.
    fn distance(&self, a: impl Point2d, b: impl Point2d) -> OrderedFloat<f32>;
}
