use crate::{
    mem::{Semi, Wrap},
    traits::*,
};

use alloc::collections::{btree_map, BTreeMap};
use core::{borrow::Borrow, iter::FusedIterator};

pub struct BTreeKind {}

impl<K, V> MapKind<K, V> for BTreeKind
where
    K: Ord,
{
    type Map = InnerBTreeMap<K, V>;
}

pub struct InnerBTreeMap<K, V> {
    map: BTreeMap<Semi<K>, Semi<V>>,
}

impl<K, V> MapBase for InnerBTreeMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K, V> New for InnerBTreeMap<K, V>
where
    K: Ord,
{
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl<K, V, Q: ?Sized> Contains<Q> for InnerBTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    fn contains(&self, key: &Q) -> bool {
        self.map.contains_key(key.wrap())
    }
}

impl<K, V, Q: ?Sized> Get<Q> for InnerBTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    fn get(&self, key: &Q) -> Option<&Semi<V>> {
        self.map.get(key.wrap())
    }
}

impl<K, V> Insert for InnerBTreeMap<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: Semi<K>, value: Semi<V>) {
        self.map.insert(key, value);
    }
}

impl<K, V> Length for InnerBTreeMap<K, V> {
    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K, V, Q: ?Sized> Remove<Q> for InnerBTreeMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    fn remove(&mut self, key: &Q) -> Option<(Semi<K>, Semi<V>)> {
        self.map.remove_entry(key.wrap())
    }
}

#[derive(Debug)]
pub struct IterOwned<K, V> {
    iter: btree_map::IntoIter<Semi<K>, Semi<V>>,
}

impl<K, V> Iterator for IterOwned<K, V> {
    type Item = (Semi<K>, Semi<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IterOwned<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V> ExactSizeIterator for IterOwned<K, V> {}

impl<K, V> FusedIterator for IterOwned<K, V> {}
