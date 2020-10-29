use crate::{
    mem::Half,
    traits::{Contains, FullInnerMap, Get, InnerMap, Remove},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Overwritten<L, R> {
    Zero,
    One((L, R)),
    Two((L, R), (L, R)),
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BiMap<LMap, RMap> {
    lmap: LMap,
    rmap: RMap,
}

impl<L, R, LMap, RMap> BiMap<LMap, RMap>
where
    LMap: FullInnerMap<Key = L, Value = R>,
    RMap: FullInnerMap<Key = R, Value = L>,
{
    pub fn contains_left<Q: ?Sized>(&self, l: &Q) -> bool
    where
        LMap: Contains<Q>,
    {
        self.lmap.contains_key(l)
    }

    pub fn contains_right<Q: ?Sized>(&self, r: &Q) -> bool
    where
        RMap: Contains<Q>,
    {
        self.rmap.contains_key(r)
    }

    pub fn get_left<Q: ?Sized>(&self, l: &Q) -> Option<&R>
    where
        LMap: Get<Q>,
    {
        self.lmap.get(l)
    }

    pub fn get_right<Q: ?Sized>(&self, r: &Q) -> Option<&L>
    where
        RMap: Get<Q>,
    {
        self.rmap.get(r)
    }

    pub fn get_entry_left<Q: ?Sized>(&self, l: &Q) -> Option<(&L, &R)>
    where
        LMap: Get<Q>,
    {
        self.lmap.get_entry(l)
    }

    pub fn get_entry_right<Q: ?Sized>(&self, r: &Q) -> Option<(&L, &R)>
    where
        RMap: Get<Q>,
    {
        self.rmap.get_entry(r).map(|(r, l)| (l, r))
    }

    pub fn remove_left<Q: ?Sized>(&mut self, l: &Q) -> Option<(L, R)>
    where
        LMap: Remove<Q>,
    {
        let (la, ra) = self.lmap.remove_entry(l)?;
        let (rb, lb) = self.rmap.remove_entry(&ra).unwrap();
        let l = Half::rejoin(la, lb);
        let r = Half::rejoin(ra, rb);
        Some((l, r))
    }

    pub fn remove_right<Q: ?Sized>(&mut self, r: &Q) -> Option<(L, R)>
    where
        RMap: Remove<Q>,
    {
        let (ra, la) = self.rmap.remove_entry(r)?;
        let (lb, rb) = self.lmap.remove_entry(&la).unwrap();
        let l = Half::rejoin(la, lb);
        let r = Half::rejoin(ra, rb);
        Some((l, r))
    }

    pub fn insert(&mut self, l: L, r: R) -> Overwritten<L, R> {
        let overwritten = match (self.remove_left(&l), self.remove_right(&r)) {
            (None, None) => Overwritten::Zero,
            (Some(pair), None) | (None, Some(pair)) => Overwritten::One(pair),
            (Some(lpair), Some(rpair)) => Overwritten::Two(lpair, rpair),
        };
        self.insert_unchecked(l, r);
        overwritten
    }

    pub fn try_insert(&mut self, l: L, r: R) -> Result<(), (L, R)> {
        if self.lmap.contains_key(&l) || self.rmap.contains_key(&r) {
            Err((l, r))
        } else {
            self.insert_unchecked(l, r);
            Ok(())
        }
    }

    fn insert_unchecked(&mut self, l: L, r: R) {
        let [la, lb] = Half::halve(l);
        let [ra, rb] = Half::halve(r);
        self.lmap.insert_unchecked(la, ra);
        self.rmap.insert_unchecked(rb, lb);
    }
}

impl<L, R, LMap, RMap> Clone for BiMap<LMap, RMap>
where
    LMap: InnerMap<Key = L, Value = R>,
    RMap: InnerMap<Key = R, Value = L>,
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        self.lmap.clone_into()
    }
}
