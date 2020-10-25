use crate::{bimap::BiMap, mem::Half};

pub trait FullInnerMap: InnerMap + Contains + Get + Remove {}

impl<T> FullInnerMap for T where T: InnerMap + Contains + Get + Remove {}

pub trait InnerMap: Default {
    type Key;
    type Value;

    fn clone_into<RMap>(&self) -> BiMap<Self, RMap>
    where
        RMap: InnerMap<Key = Self::Value, Value = Self::Key>,
        Self::Key: Clone,
        Self::Value: Clone;

    fn insert_unchecked(&mut self, k: Half<Self::Key>, v: Half<Self::Value>);
}

pub trait Contains<Q: ?Sized = <Self as InnerMap>::Key>: InnerMap {
    fn contains_key(&self, k: &Q) -> bool;
}

pub trait Get<Q: ?Sized = <Self as InnerMap>::Key>: InnerMap {
    fn get(&self, k: &Q) -> Option<&Self::Value>;
    fn get_entry(&self, k: &Q) -> Option<(&Self::Key, &Self::Value)>;
}

pub trait Remove<Q: ?Sized = <Self as InnerMap>::Key>: InnerMap {
    fn remove_entry(&self, k: &Q) -> Option<(Half<Self::Key>, Half<Self::Value>)>;
}
