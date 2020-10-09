use crate::{
    mem::{Semi, Wrap},
    traits::*,
};

use std::{
    borrow::Borrow,
    collections::{hash_map, hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
    iter::FusedIterator,
    marker::PhantomData,
};

pub struct HashKind<S = RandomState> {
    marker: PhantomData<S>,
}

impl<K, V, S> MapKind<K, V> for HashKind<S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Map = InnerHashMap<K, V, S>;
}

pub struct InnerHashMap<K, V, S = RandomState> {
    map: HashMap<Semi<K>, Semi<V>, S>,
}

impl<K, V, S> InnerHashMap<K, V, S> {
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }
}

impl<K, V, S> MapBase for InnerHashMap<K, V, S> {
    type Key = K;
    type Value = V;
}

impl<K, V, S> New for InnerHashMap<K, V, S>
where
    S: BuildHasher + Default,
{
    fn new() -> Self {
        Self {
            map: HashMap::with_hasher(S::default()),
        }
    }
}

impl<K, V, S> Length for InnerHashMap<K, V, S> {
    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K, V, S, Q> Contains<Q> for InnerHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher,
{
    fn contains(&self, value: &Q) -> bool {
        self.map.contains_key(value.wrap())
    }
}

impl<K, V, S, Q> Get<Q> for InnerHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher,
{
    fn get(&self, value: &Q) -> Option<&Semi<V>> {
        self.map.get(value.wrap())
    }
}

impl<K, V, S> Insert for InnerHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn insert(&mut self, key: Semi<K>, value: Semi<V>) {
        self.map.insert(key, value);
    }
}

impl<K, V, S, Q> Remove<Q> for InnerHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher,
{
    fn remove(&mut self, value: &Q) -> Option<(Semi<K>, Semi<V>)> {
        self.map.remove_entry(value.wrap())
    }
}

impl<K, V, S> IterateOwned for InnerHashMap<K, V, S> {
    type IterOwned = IterOwned<K, V>;

    fn iter_owned(self) -> Self::IterOwned {
        IterOwned {
            iter: self.map.into_iter(),
        }
    }
}

impl<'a, K, V, S> IterateRef<'a> for InnerHashMap<K, V, S>
where
    Self: 'a,
{
    type IterRef = IterRef<'a, K, V>;

    fn iter_ref(&'a self) -> Self::IterRef {
        IterRef {
            iter: self.map.iter(),
        }
    }
}

#[derive(Debug)]
pub struct IterOwned<K, V> {
    iter: hash_map::IntoIter<Semi<K>, Semi<V>>,
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

impl<K, V> ExactSizeIterator for IterOwned<K, V> {}

impl<K, V> FusedIterator for IterOwned<K, V> {}

#[derive(Debug)]
pub struct IterRef<'a, K, V> {
    iter: hash_map::Iter<'a, Semi<K>, Semi<V>>,
}

impl<'a, K, V> Clone for IterRef<'a, K, V> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K, V> Iterator for IterRef<'a, K, V> {
    type Item = (&'a Semi<K>, &'a Semi<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for IterRef<'a, K, V> {}

impl<'a, K, V> FusedIterator for IterRef<'a, K, V> {}
