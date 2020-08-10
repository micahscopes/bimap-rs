#![no_std]

extern crate alloc;

pub mod bimap;
pub mod btree;
pub mod map;
pub mod mem;

use crate::{bimap::RawBiMap, btree::BTreeKind, map::MapKind};

pub type BiMap<L, R, LK, RK> = RawBiMap<<LK as MapKind<L, R>>::Map, <RK as MapKind<R, L>>::Map>;

pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;
