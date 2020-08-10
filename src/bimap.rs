use crate::{btree::InnerBTreeMap, mem::Ref, map::{Map, FullMap, Contains, Iterate, Get}};

use core::{borrow::Borrow, ops::RangeBounds};

pub struct RawBiMap<LM, RM> {
    left_map: LM,
    right_map: RM,
}

impl<L, R, LM, RM> RawBiMap<LM, RM>
where
    LM: FullMap<Key = L, Value = R>,
    RM: FullMap<Key = R, Value = L>,
{
    pub fn new() -> Self {
        Self {
            left_map: LM::default(),
            right_map: RM::default(),
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
        LM: Contains<Q>,
    {
        self.left_map.contains(left)
    }

    pub fn left_get<Q: ?Sized>(&self, left: &Q) -> Option<&R>
    where
        LM: Get<Q>,
    {
        self.left_map.get(left)
    }

    pub fn left_get_entry<Q: ?Sized>(&self, left: &Q) -> Option<(&L, &R)>
    where
        LM: Get<Q>,
    {
        self.left_map.get_entry(left)
    }

    pub fn left_iter<'a>(&'a self) -> LeftIter<'a, LM>
    where
        LM: Iterate<'a>,
    {
        LeftIter {
            iter: self.left_map.iter(),
        }
    }

    pub fn right_iter<'a>(&'a self) -> RightIter<'a, RM>
    where
        RM: Iterate<'a>,
    {
        RightIter {
            iter: self.right_map.iter(),
        }
    }

    fn insert_unchecked(&mut self, left: L, right: R) {
        let (left_a, left_b) = Ref::share(left);
        let (right_a, right_b) = Ref::share(right);
        self.left_map.insert(left_a, right_a);
        self.right_map.insert(right_b, left_b);
    }
}

impl<L, R, RM> RawBiMap<InnerBTreeMap<L, R>, RM>
where
    RM: FullMap<Key = R, Value = L>,
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

pub struct LeftRange<'a, L, R> {
    iter: crate::btree::Range<'a, L, R>,
}

impl<'a, L, R> Iterator for LeftRange<'a, L, R> {
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<(&'a L, &'a R)> {
        self.iter.next()
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
