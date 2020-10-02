use crate::{
    map::traits::*,
    mem::{deref_pair, Semi, Wrap},
};

use alloc::collections::{btree_map, BTreeMap};
use core::{borrow::Borrow, iter::FusedIterator, ops::Deref};

/// The `MapKind` representing an `InnerBTreeMap`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BTreeKind {}

impl<K, V> MapKind<K, V> for BTreeKind
where
    K: Ord,
{
    type Map = InnerBTreeMap<K, V>;
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InnerBTreeMap<K, V> {
    map: BTreeMap<Semi<K>, Semi<V>>,
}

impl<K, V> Default for InnerBTreeMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self {
            map: BTreeMap::default(),
        }
    }
}

impl<K, V> Map for InnerBTreeMap<K, V> {
    type Key = K;
    type Value = V;
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
    fn get_entry(&self, key: &Q) -> Option<(&K, &V)> {
        self.map.get_key_value(key.wrap()).map(deref_pair)
    }

    fn get(&self, key: &Q) -> Option<&V> {
        self.map.get(key.wrap()).map(Deref::deref)
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

impl<'a, K: 'a, V: 'a> Iterate<'a> for InnerBTreeMap<K, V> {
    type Iter = Iter<'a, K, V>;

    fn iter(&'a self) -> Self::Iter {
        Iter {
            iter: self.map.iter(),
        }
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

impl<K, V> IntoIterate for InnerBTreeMap<K, V> {
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, K, V> {
    iter: btree_map::Iter<'a, Semi<K>, Semi<V>>,
}

impl<'a, K, V> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(deref_pair)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(deref_pair)
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

#[derive(Debug)]
pub struct IntoIter<K, V> {
    iter: btree_map::IntoIter<Semi<K>, Semi<V>>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (Semi<K>, Semi<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for IntoIter<K, V> {}

#[derive(Debug)]
pub struct Range<'a, K, V> {
    iter: btree_map::Range<'a, Semi<K>, Semi<V>>,
}

impl<'a, K, V> Clone for Range<'a, K, V> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K, V> Iterator for Range<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(deref_pair)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Range<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(deref_pair)
    }
}

impl<'a, K, V> FusedIterator for Range<'a, K, V> {}
