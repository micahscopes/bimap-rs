#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

pub mod bimap;
pub mod map;
pub mod mem;

use crate::{bimap::BiMap, map::btree::BTreeKind};

/// A `BiMap` that uses a `BTreeMap` for both inner maps.
pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;
