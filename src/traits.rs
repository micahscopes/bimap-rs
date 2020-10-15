use crate::mem::{KeyRef, ValueRef};

pub trait MapBase {
    type Key;
    type Value;

    fn new() -> Self;
}

pub trait Map: MapBase + Length + Contains + Get + Insert + Remove + IterateOwned {}

impl<T> Map for T where T: MapBase + Length + Contains + Get + Insert + Remove + IterateOwned {}

pub trait MapKind<K, V> {
    type Map: MapBase<Key = K, Value = V>;
}

pub trait Length {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait Contains<Q: ?Sized = <Self as MapBase>::Key>: MapBase {
    fn contains(&self, key: &Q) -> bool;
}

pub trait Get<Q: ?Sized = <Self as MapBase>::Key>: MapBase {
    fn get(&self, key: &Q) -> Option<&ValueRef<Self::Value>>;
}

pub trait Insert: MapBase {
    fn insert(&mut self, key: KeyRef<Self::Key>, value: ValueRef<Self::Value>);
}

pub trait Remove<Q: ?Sized = <Self as MapBase>::Key>: MapBase {
    fn remove(&mut self, key: &Q) -> Option<(KeyRef<Self::Key>, ValueRef<Self::Value>)>;
}

pub trait IterateRef<'a>: MapBase + 'a {
    type IterRef: Iterator<Item = (&'a KeyRef<Self::Key>, &'a ValueRef<Self::Value>)>;

    fn iter_ref(&'a self) -> Self::IterRef;
}

pub trait IterateOwned: MapBase {
    type IterOwned: Iterator<Item = (KeyRef<Self::Key>, ValueRef<Self::Value>)>;

    fn iter_owned(self) -> Self::IterOwned;
}
