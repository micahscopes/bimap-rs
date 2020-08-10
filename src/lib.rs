#![no_std]

extern crate alloc;

pub mod bimap;
pub mod btree;
pub mod map;
pub mod mem;

pub use crate::{bimap::BiMap, btree::BTreeKind};

pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;
