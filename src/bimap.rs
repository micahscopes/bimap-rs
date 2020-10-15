use crate::{maps::btree::BTreeKind, traits::*};

#[cfg(feature = "std")]
use crate::maps::hash::HashKind;

use core::{iter::FromIterator, marker::PhantomData, ops::Deref};

pub type BiMap<L, R, LKind, RKind> =
    RawBiMap<<LKind as MapKind<L, R>>::Map, <RKind as MapKind<R, L>>::Map>;

pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;

#[cfg(feature = "std")]
pub type BiHashMap<L, R> = BiMap<L, R, HashKind, HashKind>;

pub enum Overwritten<L, R> {
    Zero,
    One((L, R)),
    Two((L, R), (L, R)),
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RawBiMap<LMap, RMap> {
    lmap: LMap,
    rmap: RMap,
}

impl<L, R, LMap, RMap> RawBiMap<LMap, RMap>
where
    LMap: Map<Key = L, Value = R>,
    RMap: Map<Key = R, Value = L>,
{
    pub fn new() -> Self {
        Self {
            lmap: LMap::new(),
            rmap: RMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.lmap.len(), self.rmap.len());
        self.lmap.len()
    }

    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(self.lmap.is_empty(), self.rmap.is_empty());
        self.lmap.is_empty()
    }

    pub fn iter_left<'a>(&'a self) -> IterLeft<'a, LMap, RMap>
    where
        LMap: IterateRef<'a>,
    {
        IterLeft {
            iter: self.lmap.iter_ref(),
            marker: PhantomData,
        }
    }

    pub fn iter_right<'a>(&'a self) -> IterRight<'a, LMap, RMap>
    where
        RMap: IterateRef<'a>,
    {
        IterRight {
            iter: self.rmap.iter_ref(),
            marker: PhantomData,
        }
    }

    pub fn get_left<Q: ?Sized>(&self, left: &Q) -> Option<&R>
    where
        LMap: Get<Q>,
    {
        self.lmap.get(left).map(Deref::deref)
    }

    pub fn get_right<Q: ?Sized>(&self, right: &Q) -> Option<&L>
    where
        RMap: Get<Q>,
    {
        self.rmap.get(right).map(Deref::deref)
    }

    pub fn remove_left<Q: ?Sized>(&mut self, left: &Q) -> Option<(L, R)>
    where
        LMap: Remove<Q>,
    {
        let (lkey, rvalue) = self.lmap.remove(left)?;
        let (rkey, lvalue) = self.rmap.remove(&rvalue).unwrap();
        let l = crate::mem::unite(lkey, lvalue);
        let r = crate::mem::unite(rkey, rvalue);
        Some((l, r))
    }

    pub fn remove_right<Q: ?Sized>(&mut self, right: &Q) -> Option<(L, R)>
    where
        RMap: Remove<Q>,
    {
        let (rkey, lvalue) = self.rmap.remove(right)?;
        let (lkey, rvalue) = self.lmap.remove(&lvalue).unwrap();
        let l = crate::mem::unite(lkey, lvalue);
        let r = crate::mem::unite(rkey, rvalue);
        Some((l, r))
    }

    pub fn try_insert(&mut self, left: L, right: R) -> Result<(), (L, R)> {
        if self.lmap.contains(&left) || self.rmap.contains(&right) {
            Err((left, right))
        } else {
            unsafe {
                self.insert_unchecked(left, right);
            }
            Ok(())
        }
    }

    pub fn insert(&mut self, left: L, right: R) -> Overwritten<L, R> {
        let overwritten = match (self.remove_left(&left), self.remove_right(&right)) {
            (None, None) => Overwritten::Zero,
            (Some(pair), None) | (None, Some(pair)) => Overwritten::One(pair),
            (Some(lpair), Some(rpair)) => Overwritten::Two(lpair, rpair),
        };

        unsafe {
            self.insert_unchecked(left, right);
        }

        overwritten
    }

    pub unsafe fn insert_unchecked(&mut self, left: L, right: R) {
        let (lkey, rvalue) = crate::mem::split(left);
        let (rkey, lvalue) = crate::mem::split(right);
        self.lmap.insert(lkey, lvalue);
        self.rmap.insert(rkey, rvalue);
    }
}

impl<L, R, LMap, RMap> FromIterator<(L, R)> for RawBiMap<LMap, RMap>
where
    LMap: Map<Key = L, Value = R>,
    RMap: Map<Key = R, Value = L>,
{
    fn from_iter<I: IntoIterator<Item = (L, R)>>(iter: I) -> Self {
        let mut bimap = RawBiMap::new();
        for (l, r) in iter {
            bimap.insert(l, r);
        }
        bimap
    }
}

pub struct IterLeft<'a, LMap, RMap>
where
    LMap: Map + IterateRef<'a>,
    RMap: Map<Key = LMap::Value, Value = LMap::Key>,
{
    iter: LMap::IterRef,
    marker: PhantomData<&'a (LMap, RMap)>,
}

impl<'a, L: 'a, R: 'a, LMap, RMap> Iterator for IterLeft<'a, LMap, RMap>
where
    LMap: Map<Key = L, Value = R> + IterateRef<'a>,
    RMap: Map<Key = R, Value = L>,
{
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(deref_pair)
    }
}

pub struct IterRight<'a, LMap, RMap>
where
    LMap: Map,
    RMap: Map<Key = LMap::Value, Value = LMap::Key> + IterateRef<'a>,
{
    iter: RMap::IterRef,
    marker: PhantomData<&'a (LMap, RMap)>,
}

impl<'a, L: 'a, R: 'a, LMap, RMap> Iterator for IterRight<'a, LMap, RMap>
where
    LMap: Map<Key = L, Value = R>,
    RMap: Map<Key = R, Value = L> + IterateRef<'a>,
{
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(deref_pair).map(swap_pair)
    }
}

fn deref_pair<'a, L: Deref, R: Deref>((l, r): (&'a L, &'a R)) -> (&'a L::Target, &'a R::Target) {
    (&*l, &*r)
}

fn swap_pair<L, R>((l, r): (L, R)) -> (R, L) {
    (r, l)
}
