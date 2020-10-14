use crate::mem::Semi;

pub trait MapBase {
    type Key;
    type Value;
}

pub trait Map: MapBase + New + Length + Contains + Get + Insert + Remove + IterateOwned {}

impl<T> Map for T where T: MapBase + New + Length + Contains + Get + Insert + Remove + IterateOwned {}

pub trait MapKind<K, V> {
    type Map: MapBase<Key = K, Value = V>;
}

pub trait New: MapBase {
    fn new() -> Self;
}

pub trait Length {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait Contains<Q: ?Sized = <Self as MapBase>::Key>: MapBase {
    fn contains(&self, key: &Q) -> bool;
}

pub trait Get<Q: ?Sized = <Self as MapBase>::Key>: MapBase {
    fn get(&self, key: &Q) -> Option<&Semi<Self::Value>>;
}

pub trait Insert: MapBase {
    fn insert(&mut self, key: Semi<Self::Key>, value: Semi<Self::Value>);
}

pub trait Remove<Q: ?Sized = <Self as MapBase>::Key>: MapBase {
    fn remove(&mut self, key: &Q) -> Option<(Semi<Self::Key>, Semi<Self::Value>)>;
}

pub trait IterateRef<'a>: MapBase + 'a {
    type IterRef: Iterator<Item = (&'a Semi<Self::Key>, &'a Semi<Self::Value>)>;

    fn iter_ref(&'a self) -> Self::IterRef;
}

pub trait IterateOwned: MapBase {
    type IterOwned: Iterator<Item = (Semi<Self::Key>, Semi<Self::Value>)>;

    fn iter_owned(self) -> Self::IterOwned;
}
