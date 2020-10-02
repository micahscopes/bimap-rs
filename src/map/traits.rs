use crate::mem::Semi;

/// The base trait representing any map that can be used in a `BiMap`.
///
/// This trait intentionally has no methods, only the associated types `Key` and `Value`.
pub trait Map {
    type Key;
    type Value;
}

pub trait MapKind<K, V>
where
    K: Eq,
{
    type Map: Default + Map<Key = K, Value = V> + Contains<K> + Get<K> + Insert + Length + Remove<K>;
}

pub trait Length: Map {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Contains<Q: ?Sized>: Map {
    fn contains(&self, key: &Q) -> bool;
}

pub trait Get<Q: ?Sized>: Map {
    fn get_entry(&self, key: &Q) -> Option<(&Self::Key, &Self::Value)>;

    fn get(&self, key: &Q) -> Option<&Self::Value> {
        self.get_entry(key).map(|(_, v)| v)
    }
}

pub trait Insert: Map {
    fn insert(&mut self, key: Semi<Self::Key>, value: Semi<Self::Value>);
}

pub trait Iterate<'a>: Map + 'a {
    type Iter: Iterator<Item = (&'a Self::Key, &'a Self::Value)>;

    fn iter(&'a self) -> Self::Iter;
}

pub trait IntoIterate: Map {
    type IntoIter: Iterator<Item = (Semi<Self::Key>, Semi<Self::Value>)>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait Remove<Q: ?Sized>: Map {
    fn remove(&mut self, key: &Q) -> Option<(Semi<Self::Key>, Semi<Self::Value>)>;
}
