use crate::{
    btree::{self, BTreeKind},
    map::*,
    mem::Ref,
};

use core::{borrow::Borrow, ops::RangeBounds};

pub struct BiMap<L, R, LK, RK>
where
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    left_map: LK::Map,
    right_map: RK::Map,
}

impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    /// Creates an empty bimap.
    pub fn new() -> Self {
        Self {
            left_map: Default::default(),
            right_map: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.left_map.len()
    }

    pub fn try_insert(&mut self, left: L, right: R) -> Result<(), (L, R)> {
        if self.left_map.contains(&left) || self.right_map.contains(&right) {
            Err((left, right))
        } else {
            self.insert_unchecked(left, right);
            Ok(())
        }
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

    pub fn left_get_entry<Q: ?Sized>(&self, left: &Q) -> Option<(&L, &R)>
    where
        LK::Map: Get<Q>,
    {
        self.left_map.get_entry(left)
    }

    pub fn left_iter<'a>(&'a self) -> LeftIter<'a, LK::Map>
    where
        LK::Map: Iterate<'a>,
    {
        LeftIter {
            iter: self.left_map.iter(),
        }
    }

    pub fn left_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(L, R)>
    where
        LK::Map: Remove<Q>,
    {
        let (left_a, right_a) = self.left_map.remove(key)?;
        let (right_b, left_b) = self.right_map.remove(&right_a).unwrap();
        Some(Self::rejoin_pair((left_a, right_a), (left_b, right_b)))
    }

    pub fn right_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(L, R)>
    where
        RK::Map: Remove<Q>,
    {
        let (right_a, left_a) = self.right_map.remove(key)?;
        let (left_b, right_b) = self.left_map.remove(&left_a).unwrap();
        Some(Self::rejoin_pair((left_a, right_a), (left_b, right_b)))
    }

    pub fn right_iter<'a>(&'a self) -> RightIter<'a, RK::Map>
    where
        RK::Map: Iterate<'a>,
    {
        RightIter {
            iter: self.right_map.iter(),
        }
    }

    fn insert_unchecked(&mut self, left: L, right: R) {
        let ((left_a, right_a), (left_b, right_b)) = Self::share_pair(left, right);
        self.left_map.insert(left_a, right_a);
        self.right_map.insert(right_b, left_b);
    }

    fn share_pair(left: L, right: R) -> ((Ref<L>, Ref<R>), (Ref<L>, Ref<R>)) {
        let (left_a, left_b) = Ref::share(left);
        let (right_a, right_b) = Ref::share(right);
        ((left_a, right_a), (left_b, right_b))
    }

    fn rejoin_pair(a: (Ref<L>, Ref<R>), b: (Ref<L>, Ref<R>)) -> (L, R) {
        let (left_a, right_a) = a;
        let (left_b, right_b) = b;
        (Ref::rejoin(left_a, left_b), Ref::rejoin(right_a, right_b))
    }
}

impl<L, R, RK> BiMap<L, R, BTreeKind, RK>
where
    RK: MapKind<R, L>,
    L: Ord,
{
    pub fn left_range<A, Q: ?Sized>(&self, range: A) -> LeftRange<'_, L, R>
    where
        L: Ord + Borrow<Q>,
        A: RangeBounds<Q>,
        Q: Ord,
    {
        LeftRange {
            iter: self.left_map.range(range),
        }
    }
}

pub struct LeftIter<'a, LM>
where
    LM: Iterate<'a>,
{
    iter: LM::Iter,
}

impl<'a, L: 'a, R: 'a, LM> Iterator for LeftIter<'a, LM>
where
    LM: Map<Key = L, Value = R> + Iterate<'a>,
{
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct LeftRange<'a, L, R> {
    iter: btree::Range<'a, L, R>,
}

impl<'a, L, R> Iterator for LeftRange<'a, L, R> {
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<(&'a L, &'a R)> {
        self.iter.next()
    }
}

pub struct RightIter<'a, RM>
where
    RM: Iterate<'a>,
{
    iter: RM::Iter,
}

impl<'a, L: 'a, R: 'a, RM> Iterator for RightIter<'a, RM>
where
    RM: Map<Key = R, Value = L> + Iterate<'a>,
{
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (v, k))
    }
}
