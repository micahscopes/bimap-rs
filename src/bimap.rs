use crate::{maps::btree::BTreeKind, mem::Semi, traits::*};

#[cfg(feature = "std")]
use crate::maps::hash::HashKind;

use core::{iter::FromIterator, marker::PhantomData, ops::Deref};

fn deref_pair<'a, L: Deref, R: Deref>((l, r): (&'a L, &'a R)) -> (&'a L::Target, &'a R::Target) {
    (&*l, &*r)
}

fn swap_pair<L, R>((l, r): (L, R)) -> (R, L) {
    (r, l)
}

pub type BiMap<L, R, LKind, RKind> =
    RawBiMap<<LKind as MapKind<L, R>>::Map, <RKind as MapKind<R, L>>::Map>;

pub type BiBTreeMap<L, R> = BiMap<L, R, BTreeKind, BTreeKind>;

#[cfg(feature = "std")]
pub type BiHashMap<L, R> = BiMap<L, R, HashKind, HashKind>;

pub enum Overwritten<L, R> {
    None,
    One((L, R)),
    Two((L, R), (L, R)),
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RawBiMap<LMap, RMap> {
    left_map: LMap,
    right_map: RMap,
}

impl<L, R, LMap, RMap> RawBiMap<LMap, RMap>
where
    LMap: Map<Key = L, Value = R>,
    RMap: Map<Key = R, Value = L>,
{
    pub fn new() -> Self {
        Self {
            left_map: LMap::new(),
            right_map: RMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.left_map.len(), self.right_map.len());
        self.left_map.len()
    }

    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(self.left_map.is_empty(), self.right_map.is_empty());
        self.left_map.is_empty()
    }

    pub fn iter_left<'a>(&'a self) -> IterLeft<'a, LMap::IterRef>
    where
        LMap: IterateRef<'a>,
    {
        IterLeft {
            iter: self.left_map.iter_ref(),
            marker: PhantomData,
        }
    }

    pub fn iter_right<'a>(&'a self) -> IterRight<'a, RMap::IterRef>
    where
        RMap: IterateRef<'a>,
    {
        IterRight {
            iter: self.right_map.iter_ref(),
            marker: PhantomData,
        }
    }

    pub fn get_left<Q: ?Sized>(&self, left: &Q) -> Option<&R>
    where
        LMap: Get<Q>,
    {
        self.left_map.get(left).map(Deref::deref)
    }

    pub fn get_right<Q: ?Sized>(&self, right: &Q) -> Option<&L>
    where
        RMap: Get<Q>,
    {
        self.right_map.get(right).map(Deref::deref)
    }

    pub fn remove_left<Q: ?Sized>(&mut self, left: &Q) -> Option<(L, R)>
    where
        LMap: Remove<Q>,
    {
        let (la, rb) = self.left_map.remove(left)?;
        let (ra, lb) = self.right_map.remove(&rb)?;
        Some((Semi::reunite([la, lb]), Semi::reunite([ra, rb])))
    }

    pub fn remove_right<Q: ?Sized>(&mut self, right: &Q) -> Option<(L, R)>
    where
        RMap: Remove<Q>,
    {
        let (ra, lb) = self.right_map.remove(right)?;
        let (la, rb) = self.left_map.remove(&lb)?;
        Some((Semi::reunite([la, lb]), Semi::reunite([ra, rb])))
    }

    pub fn try_insert(&mut self, left: L, right: R) -> Result<(), (L, R)> {
        if self.left_map.contains(&left) || self.right_map.contains(&right) {
            Err((left, right))
        } else {
            self.insert_unchecked(left, right);
            Ok(())
        }
    }

    pub fn insert(&mut self, left: L, right: R) -> Overwritten<L, R> {
        let overwritten = match (self.remove_left(&left), self.remove_right(&right)) {
            (None, None) => Overwritten::None,
            (Some(pair), None) | (None, Some(pair)) => Overwritten::One(pair),
            (Some(lpair), Some(rpair)) => Overwritten::Two(lpair, rpair),
        };

        self.insert_unchecked(left, right);

        overwritten
    }

    fn insert_unchecked(&mut self, left: L, right: R)
    where
        LMap: Insert,
        RMap: Insert,
    {
        let [la, lb] = Semi::share(left);
        let [ra, rb] = Semi::share(right);
        self.left_map.insert(la, rb);
        self.right_map.insert(ra, lb);
    }
}

impl<L, R, LMap, RMap> Clone for RawBiMap<LMap, RMap>
where
    LMap: Map<Key = L, Value = R> + for<'a> IterateRef<'a>,
    RMap: Map<Key = R, Value = L>,
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        let mut bimap = RawBiMap::new();
        for (l, r) in self.iter_left() {
            bimap.insert_unchecked(l.clone(), r.clone());
        }
        bimap
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

pub struct IterLeft<'a, I> {
    iter: I,
    marker: PhantomData<&'a ()>,
}

impl<'a, I, L: 'a, R: 'a> Iterator for IterLeft<'a, I>
where
    I: Iterator<Item = (&'a Semi<L>, &'a Semi<R>)>,
{
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(deref_pair)
    }
}

pub struct IterRight<'a, I> {
    iter: I,
    marker: PhantomData<&'a ()>,
}

impl<'a, I, L: 'a, R: 'a> Iterator for IterRight<'a, I>
where
    I: Iterator<Item = (&'a Semi<R>, &'a Semi<L>)>,
{
    type Item = (&'a L, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(deref_pair).map(swap_pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut bimap = BiHashMap::new();
        bimap.insert("hello".to_owned(), 10);
        assert_eq!(bimap.get_left("hello"), Some(&10));
    }
}
