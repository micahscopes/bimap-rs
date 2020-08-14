//! A generic bidirectional map.

use crate::{
    map::{
        btree::{self, BTreeKind},
        traits::*,
    },
    mem::Semi,
};

use core::{
    borrow::Borrow,
    fmt,
    iter::{FromIterator, FusedIterator},
    ops::RangeBounds,
};

/// A generic bidirectional map.
pub struct BiMap<L, R, LK, RK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    left_map: LK::Map,
    right_map: RK::Map,
}

/// # Creating a `BiMap`
impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    /// Creates an empty `BiMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bimap::BiBTreeMap as BiMap;
    ///
    /// let bimap = BiMap::<char, u8>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            left_map: Default::default(),
            right_map: Default::default(),
        }
    }

    pub fn left_clone<'a>(&'a self) -> Self
    where
        LK::Map: Iterate<'a>,
        L: Clone,
        R: Clone,
    {
        let mut cloned = Self::new();
        for (l, r) in self.left_iter() {
            cloned.insert_unchecked(l.clone(), r.clone());
        }
        cloned
    }

    pub fn right_clone<'a>(&'a self) -> Self
    where
        RK::Map: Iterate<'a>,
        L: Clone,
        R: Clone,
    {
        let mut cloned = Self::new();
        for (l, r) in self.right_iter() {
            cloned.insert_unchecked(l.clone(), r.clone());
        }
        cloned
    }
}

/// # Inserting elements
impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    pub fn try_insert(&mut self, left: L, right: R) -> Result<(), (L, R)> {
        if self.left_map.contains(&left) || self.right_map.contains(&right) {
            Err((left, right))
        } else {
            self.insert_unchecked(left, right);
            Ok(())
        }
    }

    pub fn insert(&mut self, left: L, right: R) -> Overwritten<L, R> {
        let overwritten = match (self.left_remove(&left), self.right_remove(&right)) {
            (None, None) => Overwritten::Neither,

            (Some(left_pair), None) if left_pair.1 == right => Overwritten::Pair(left_pair),

            (Some(left_pair), None) => Overwritten::Left(left_pair),

            (None, Some(right_pair)) => Overwritten::Right(right_pair),

            (Some(left_pair), Some(right_pair)) => Overwritten::Both(left_pair, right_pair),
        };

        self.insert_unchecked(left, right);

        overwritten
    }

    fn insert_unchecked(&mut self, left: L, right: R) {
        let [left_0, left_1] = Semi::share(left);
        let [right_0, right_1] = Semi::share(right);
        self.left_map.insert(left_0, right_1);
        self.right_map.insert(right_0, left_1);
    }
}

/// # Checking membership
impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    pub fn len(&self) -> usize {
        self.left_map.len()
    }

    pub fn left_contains<Q: ?Sized>(&self, left: &Q) -> bool
    where
        LK::Map: Contains<Q>,
    {
        self.left_map.contains(left)
    }

    pub fn left_get<Q: ?Sized>(&self, left: &Q) -> Option<&R>
    where
        LK::Map: Get<Q>,
    {
        self.left_map.get(left)
    }

    pub fn left_get_pair<Q: ?Sized>(&self, left: &Q) -> Option<(&L, &R)>
    where
        LK::Map: Get<Q>,
    {
        self.left_map.get_entry(left)
    }
}

/// # Removing elements
impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    pub fn left_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(L, R)>
    where
        LK::Map: Remove<Q>,
    {
        let (left_0, right_1) = self.left_map.remove(key)?;
        let (right_0, left_1) = self.right_map.remove(&right_1).unwrap();
        let left = Semi::reunite([left_0, left_1]);
        let right = Semi::reunite([right_0, right_1]);
        Some((left, right))
    }

    pub fn right_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(L, R)>
    where
        RK::Map: Remove<Q>,
    {
        let (right_0, left_1) = self.right_map.remove(key)?;
        let (left_0, right_1) = self.left_map.remove(&left_1).unwrap();
        let left = Semi::reunite([left_0, left_1]);
        let right = Semi::reunite([right_0, right_1]);
        Some((left, right))
    }
}

/// # Iterating over a `BiMap`
impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    pub fn left_iter<'a>(&'a self) -> LeftIter<'a, L, R, LK>
    where
        LK::Map: Iterate<'a>,
    {
        LeftIter {
            iter: self.left_map.iter(),
        }
    }

    pub fn right_iter<'a>(&'a self) -> RightIter<'a, L, R, RK>
    where
        RK::Map: Iterate<'a>,
    {
        RightIter {
            iter: self.right_map.iter(),
        }
    }
}

/// # Methods when the left map has kind `BTreeKind`
impl<L, R, RK> BiMap<L, R, BTreeKind, RK>
where
    L: Eq,
    R: Eq,
    RK: MapKind<R, L>,
    L: Ord,
{
    pub fn left_range<A, Q: ?Sized>(&self, _range: A) -> LeftRange<'_, L, R>
    where
        L: Ord + Borrow<Q>,
        A: RangeBounds<Q>,
        Q: Ord,
    {
        todo!()
    }
}

mod _bimap_traits {
    use super::*;

    impl<L, R, LK, RK> Clone for BiMap<L, R, LK, RK>
    where
        L: Eq + Clone,
        R: Eq + Clone,
        LK: MapKind<L, R>,
        RK: MapKind<R, L>,
        LK::Map: for<'a> Iterate<'a>,
        RK::Map: for<'a> Iterate<'a>,
    {
        fn clone(&self) -> Self {
            let mut clone = BiMap::new();
            for (l, r) in self.left_iter() {
                clone.insert_unchecked(l.clone(), r.clone());
            }
            clone
        }
    }

    impl<L, R, LK, RK> fmt::Debug for BiMap<L, R, LK, RK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        RK: MapKind<R, L>,
        LK::Map: fmt::Debug,
        RK::Map: fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("BiMap")
                .field("left_map", &self.left_map)
                .field("right_map", &self.right_map)
                .finish()
        }
    }

    impl<L, R, LK, RK> FromIterator<(L, R)> for BiMap<L, R, LK, RK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        RK: MapKind<R, L>,
    {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = (L, R)>,
        {
            let mut bimap = Self::new();
            bimap.extend(iter);
            bimap
        }
    }

    impl<L, R, LK, RK> Extend<(L, R)> for BiMap<L, R, LK, RK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        RK: MapKind<R, L>,
    {
        fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item = (L, R)>,
        {
            for (left, right) in iter {
                self.insert(left, right);
            }
        }
    }
}

pub enum Overwritten<L, R> {
    Neither,
    Left((L, R)),
    Right((L, R)),
    Pair((L, R)),
    Both((L, R), (L, R)),
}

pub struct LeftIter<'a, L, R, LK>
where
    L: Eq,
    R: Eq,
    LK: MapKind<L, R>,
    LK::Map: Iterate<'a>,
{
    iter: <LK::Map as Iterate<'a>>::Iter,
}

mod _left_iter {
    use super::*;

    impl<'a, L: 'a, R: 'a, LK> Iterator for LeftIter<'a, L, R, LK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        LK::Map: Iterate<'a>,
    {
        type Item = (&'a L, &'a R);

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    }

    impl<'a, L: 'a, R: 'a, LK> ExactSizeIterator for LeftIter<'a, L, R, LK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        LK::Map: Iterate<'a>,
        <LK::Map as Iterate<'a>>::Iter: ExactSizeIterator,
    {
        fn len(&self) -> usize {
            self.iter.len()
        }
    }

    impl<'a, L: 'a, R: 'a, LK> FusedIterator for LeftIter<'a, L, R, LK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        LK::Map: Iterate<'a>,
        <LK::Map as Iterate<'a>>::Iter: FusedIterator,
    {
    }

    impl<'a, L: 'a, R: 'a, LK> DoubleEndedIterator for LeftIter<'a, L, R, LK>
    where
        L: Eq,
        R: Eq,
        LK: MapKind<L, R>,
        LK::Map: Iterate<'a>,
        <LK::Map as Iterate<'a>>::Iter: DoubleEndedIterator,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }
}

pub struct RightIter<'a, L, R, RK>
where
    L: Eq,
    R: Eq,
    RK: MapKind<R, L>,
    RK::Map: Iterate<'a>,
{
    iter: <RK::Map as Iterate<'a>>::Iter,
}

mod _right_iter {
    use super::*;

    impl<'a, L: 'a, R: 'a, RK> Iterator for RightIter<'a, L, R, RK>
    where
        L: Eq,
        R: Eq,
        RK: MapKind<R, L>,
        RK::Map: Iterate<'a>,
    {
        type Item = (&'a L, &'a R);

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|(r, l)| (l, r))
        }
    }

    impl<'a, L: 'a, R: 'a, RK> ExactSizeIterator for RightIter<'a, L, R, RK>
    where
        L: Eq,
        R: Eq,
        RK: MapKind<R, L>,
        RK::Map: Iterate<'a>,
        <RK::Map as Iterate<'a>>::Iter: ExactSizeIterator,
    {
        fn len(&self) -> usize {
            self.iter.len()
        }
    }

    impl<'a, L: 'a, R: 'a, RK> FusedIterator for RightIter<'a, L, R, RK>
    where
        L: Eq,
        R: Eq,
        RK: MapKind<R, L>,
        RK::Map: Iterate<'a>,
        <RK::Map as Iterate<'a>>::Iter: FusedIterator,
    {
    }

    impl<'a, L: 'a, R: 'a, RK> DoubleEndedIterator for RightIter<'a, L, R, RK>
    where
        L: Eq,
        R: Eq,
        RK: MapKind<R, L>,
        RK::Map: Iterate<'a>,
        <RK::Map as Iterate<'a>>::Iter: DoubleEndedIterator,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back().map(|(r, l)| (l, r))
        }
    }
}

pub struct LeftRange<'a, L, R> {
    iter: btree::Range<'a, L, R>,
}

mod _left_range {
    use super::*;

    impl<'a, L, R> Iterator for LeftRange<'a, L, R> {
        type Item = (&'a L, &'a R);

        fn next(&mut self) -> Option<(&'a L, &'a R)> {
            self.iter.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BiBTreeMap;

    use alloc::vec::Vec;

    #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
    struct Key<'a> {
        data: &'a Vec<u8>,
    }

    #[test]
    fn test() {
        let mut bimap = BiBTreeMap::new();

        let data = vec![1, 2, 3];
        let key = Key { data: &data };

        bimap.insert(key, 1);

        let new_data = vec![1, 2, 3];
        let new_key = Key { data: &new_data };

        assert_eq!(bimap.left_get(&new_key), Some(&1));

        let b = bimap.left_clone();

        assert_eq!(b.left_get(&new_key), Some(&1));
    }
}
