#![no_std]

extern crate alloc;

pub mod bimap;
pub mod map;
pub mod mem;

pub use crate::{bimap::BiMap, map::btree::BTreeKind};

/// A `BiMap` that uses a `BTreeMap` for both inner maps.
pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pair<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> From<(L, R)> for Pair<L, R> {
    fn from((left, right): (L, R)) -> Self {
        Self { left, right }
    }
}

impl<L, R> From<Pair<L, R>> for (L, R) {
    fn from(pair: Pair<L, R>) -> Self {
        (pair.left, pair.right)
    }
}
