use crate::mem::Ref;

pub trait Map {
    type Key;
    type Value;
}

pub trait MapKind<K, V> {
    type Map: Map<Key = K, Value = V>
        + Contains<K>
        + Get<K>
        + Insert
        + Length
        + Remove<K>
        + Default
        + Extend<(Ref<K>, Ref<V>)>;
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

pub trait Length: Map {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Insert: Map {
    fn insert(&mut self, key: Ref<Self::Key>, value: Ref<Self::Value>);
}

pub trait Iterate<'a>: Map + 'a {
    type Iter: Iterator<Item = (&'a Self::Key, &'a Self::Value)>;

    fn iter(&'a self) -> Self::Iter;
}

pub trait IntoIterate: Map {
    type IntoIter: Iterator<Item = (Ref<Self::Key>, Ref<Self::Value>)>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait Remove<Q: ?Sized>: Map {
    fn remove(&mut self, key: &Q) -> Option<(Ref<Self::Key>, Ref<Self::Value>)>;
}
