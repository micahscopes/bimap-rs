use crate::{
    map::{
        btree::{self, BTreeKind},
        traits::*,
    },
    mem::SemiRef,
};

use core::{borrow::Borrow, iter::FusedIterator, ops::RangeBounds};

pub enum Overwritten<L, R> {
    Neither,
    Left(L, R),
    Right(L, R),
    Pair(L, R),
    Both((L, R), (L, R)),
}

pub struct Pair<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Pair<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> From<Pair<L, R>> for (L, R) {
    fn from(pair: Pair<L, R>) -> Self {
        (pair.left, pair.right)
    }
}

impl<L, R> From<(L, R)> for Pair<L, R> {
    fn from((left, right): (L, R)) -> Self {
        Self { left, right }
    }
}

pub struct BiMap<L, R, LK, RK>
where
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    lmap: LK::Map,
    rmap: RK::Map,
}

impl<L, R, LK, RK> BiMap<L, R, LK, RK>
where
    LK: MapKind<L, R>,
    RK: MapKind<R, L>,
{
    /// Creates an empty bimap.
    pub fn new() -> Self {
        Self {
            lmap: Default::default(),
            rmap: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.lmap.len()
    }

    pub fn insert(&mut self, pair: Pair<L, R>) -> Overwritten<L, R> {
        let (left, right) = pair.into();
        let overwritten = match (self.lmap.remove(&left), self.rmap.remove(&right)) {
            (None, None) => Overwritten::Neither,
            _ => todo!(),
        };

        overwritten
    }

    pub fn try_insert(&mut self, pair: Pair<L, R>) -> Result<(), Pair<L, R>> {
        if self.lmap.contains(&pair.left) || self.rmap.contains(&pair.right) {
            Err(pair)
        } else {
            self.insert_unchecked(pair.left, pair.right);
            Ok(())
        }
    }

    pub fn left_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(L, R)>
    where
        LK::Map: Remove<Q>,
    {
        let (left_a, right_a) = self.lmap.remove(key)?;
        let (right_b, left_b) = self.rmap.remove(&right_a).unwrap();
        Some(Self::rejoin_pair((left_a, right_a), (left_b, right_b)))
    }

    pub fn right_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<(L, R)>
    where
        RK::Map: Remove<Q>,
    {
        let (right_a, left_a) = self.rmap.remove(key)?;
        let (left_b, right_b) = self.lmap.remove(&left_a).unwrap();
        Some(Self::rejoin_pair((left_a, right_a), (left_b, right_b)))
    }

    pub fn left_contains<Q: ?Sized>(&self, left: &Q) -> bool
    where
        LK::Map: Contains<Q>,
    {
        self.lmap.contains(left)
    }

    pub fn left_get<Q: ?Sized>(&self, left: &Q) -> Option<&R>
    where
        LK::Map: Get<Q>,
    {
        self.lmap.get(left)
    }

    pub fn left_get_entry<Q: ?Sized>(&self, left: &Q) -> Option<Pair<&L, &R>>
    where
        LK::Map: Get<Q>,
    {
        self.lmap.get_entry(left).map(Pair::from)
    }

    pub fn left_iter<'a>(&'a self) -> LeftIter<'a, L, R, LK>
    where
        LK::Map: Iterate<'a>,
    {
        LeftIter {
            iter: self.lmap.iter(),
        }
    }

    pub fn right_iter<'a>(&'a self) -> RightIter<'a, RK::Map>
    where
        RK::Map: Iterate<'a>,
    {
        RightIter {
            iter: self.rmap.iter(),
        }
    }

    fn insert_unchecked(&mut self, left: L, right: R) {
        let ((left_a, right_a), (left_b, right_b)) = Self::share_pair(left, right);
        self.lmap.insert(left_a, right_a);
        self.rmap.insert(right_b, left_b);
    }

    fn share_pair(left: L, right: R) -> ((SemiRef<L>, SemiRef<R>), (SemiRef<L>, SemiRef<R>)) {
        let (left_a, left_b) = SemiRef::share(left);
        let (right_a, right_b) = SemiRef::share(right);
        ((left_a, right_a), (left_b, right_b))
    }

    fn rejoin_pair(a: (SemiRef<L>, SemiRef<R>), b: (SemiRef<L>, SemiRef<R>)) -> (L, R) {
        let (left_a, right_a) = a;
        let (left_b, right_b) = b;
        (
            SemiRef::reunite(left_a, left_b),
            SemiRef::reunite(right_a, right_b),
        )
    }
}

impl<L, R, RK> BiMap<L, R, BTreeKind, RK>
where
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

pub struct LeftIter<'a, L, R, LK>
where
    LK: MapKind<L, R>,
    LK::Map: Iterate<'a>,
{
    iter: <LK::Map as Iterate<'a>>::Iter,
}

mod left_iter_impls {
    use super::*;

    impl<'a, L: 'a, R: 'a, LK> Iterator for LeftIter<'a, L, R, LK>
    where
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
        LK: MapKind<L, R>,
        LK::Map: Iterate<'a>,
        <LK::Map as Iterate<'a>>::Iter: FusedIterator,
    {
    }

    impl<'a, L: 'a, R: 'a, LK> DoubleEndedIterator for LeftIter<'a, L, R, LK>
    where
        LK: MapKind<L, R>,
        LK::Map: Iterate<'a>,
        <LK::Map as Iterate<'a>>::Iter: DoubleEndedIterator,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
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
