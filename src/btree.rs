use crate::{
    map::MapKind,
    mem::{Ref, Wrap, Wrapped},
};

use alloc::collections::{btree_map, BTreeMap};
use core::{
    borrow::Borrow,
    iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator},
    ops::{Bound, Deref, RangeBounds},
};

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
    map: BTreeMap<Ref<K>, Ref<V>>,
}

fn wrap_bound<T: ?Sized>(bound: Bound<&T>) -> Bound<&Wrapped<T>> {
    match bound {
        Bound::Included(x) => Bound::Included(x.wrap()),
        Bound::Excluded(x) => Bound::Excluded(x.wrap()),
        Bound::Unbounded => Bound::Unbounded,
    }
}

impl<K, V> InnerBTreeMap<K, V>
where
    K: Ord,
{
    pub fn range<A, Q: ?Sized>(&self, range: A) -> Range<'_, K, V>
    where
        K: Borrow<Q>,
        A: RangeBounds<Q>,
        Q: Ord,
    {
        let start = wrap_bound(range.start_bound());
        let end = wrap_bound(range.end_bound());
        Range {
            iter: self.map.range::<Wrapped<Q>, _>((start, end)),
        }
    }
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

impl<K, V> Extend<(Ref<K>, Ref<V>)> for InnerBTreeMap<K, V>
where
    K: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (Ref<K>, Ref<V>)>,
    {
        self.map.extend(iter);
    }
}

mod map_impls {
    use super::*;
    use crate::map::*;

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
            self.map
                .get_key_value(key.wrap())
                .map(|(k, v)| (k.deref(), v.deref()))
        }
    }

    impl<K, V> Insert for InnerBTreeMap<K, V>
    where
        K: Ord,
    {
        fn insert(&mut self, key: Ref<K>, value: Ref<V>) {
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
        fn remove(&mut self, key: &Q) -> Option<(Ref<K>, Ref<V>)> {
            self.map.remove_entry(key.wrap())
        }
    }
}

pub struct Iter<'a, K, V> {
    iter: btree_map::Iter<'a, Ref<K>, Ref<V>>,
}

mod iter_impls {
    use super::*;

    impl<'a, K, V> Iterator for Iter<'a, K, V> {
        type Item = (&'a K, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|(k, v)| (k.deref(), v.deref()))
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }

    impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back().map(|(k, v)| (k.deref(), v.deref()))
        }
    }

    impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
        fn len(&self) -> usize {
            self.iter.len()
        }
    }

    impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}
}

pub struct IntoIter<K, V> {
    iter: btree_map::IntoIter<Ref<K>, Ref<V>>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (Ref<K>, Ref<V>);

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

pub struct Range<'a, K, V> {
    iter: btree_map::Range<'a, Ref<K>, Ref<V>>,
}

impl<'a, K, V> Iterator for Range<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (k.deref(), v.deref()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
