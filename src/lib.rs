#![no_std]

extern crate alloc;

pub mod bimap;
pub mod map;
pub mod mem;

pub use crate::{
    bimap::{BiMap, Overwritten},
    map::btree::BTreeKind,
};

/// A `BiMap` that uses a `BTreeMap` for both inner maps.
pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pair<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Pair<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    pub fn swap(self) -> Pair<R, L> {
        Pair {
            left: self.right,
            right: self.left,
        }
    }
}

impl<L, R> From<Pair<L, R>> for (L, R) {
    fn from(pair: Pair<L, R>) -> Self {
        (pair.left, pair.right)
    }
}

impl<L, R> From<(L, R)> for Pair<L, R> {
    fn from((left, right): (L, R)) -> Self {
        Self { left, right }
    }
}
