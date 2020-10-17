use std::{collections::HashMap, hash::Hash, iter::FromIterator, ops::Deref, rc::Rc};

#[derive(Debug, Eq, PartialEq)]
pub struct BiMap<LMap, RMap> {
    lmap: LMap,
    rmap: RMap,
}

impl<L, R, LMap, RMap> FromIterator<(L, R)> for BiMap<LMap, RMap>
where
    LMap: InnerMap<Key = L, Value = R>,
    RMap: InnerMap<Key = R, Value = L>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (L, R)>,
    {
        let mut bimap = Self::new();
        for (l, r) in iter {
            bimap.insert(l, r);
        }
        bimap
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RefHalf<T> {
    ptr: Rc<T>,
}

impl<T> Deref for RefHalf<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<T> RefHalf<T> {
    pub fn divide(t: T) -> [Self; 2] {
        let rc = Rc::new(t);
        let other = rc.clone();
        [Self { ptr: rc }, Self { ptr: other }]
    }

    pub fn reunite(a: Self, b: Self) -> Result<T, [Self; 2]> {
        if Rc::ptr_eq(&a.ptr, &b.ptr) {
            drop(b.ptr);
            Ok(Rc::try_unwrap(a.ptr).ok().unwrap())
        } else {
            Err([a, b])
        }
    }
}

impl<L, R, LMap, RMap> BiMap<LMap, RMap>
where
    LMap: InnerMap<Key = L, Value = R>,
    RMap: InnerMap<Key = R, Value = L>,
{
    fn new() -> Self {
        Self {
            lmap: LMap::default(),
            rmap: RMap::default(),
        }
    }

    fn insert(&mut self, l: L, r: R) {
        todo!()
    }
}

pub trait InnerMap: Default {
    type Key;
    type Value;

    fn clone_into<RMap>(&self) -> BiMap<Self, RMap>
    where
        RMap: InnerMap<Key = Self::Value, Value = Self::Key>,
        Self::Key: Clone,
        Self::Value: Clone;

    fn insert(
        &mut self,
        k: RefHalf<Self::Key>,
        v: RefHalf<Self::Value>,
    ) -> Option<(RefHalf<Self::Key>, RefHalf<Self::Value>)>;

    fn try_insert(
        &mut self,
        k: RefHalf<Self::Key>,
        v: RefHalf<Self::Value>,
    ) -> Result<(), (RefHalf<Self::Key>, RefHalf<Self::Value>)>;
}

pub struct InnerHashMap<K, V> {
    map: HashMap<RefHalf<K>, RefHalf<V>>,
}

impl<K, V> Default for InnerHashMap<K, V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K, V> InnerMap for InnerHashMap<K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;

    fn insert(&mut self, k: RefHalf<K>, v: RefHalf<V>) -> Option<(RefHalf<K>, RefHalf<V>)> {
        let prev_entry = self.map.remove_entry(&k);
        self.insert(k, v);
        prev_entry
    }

    fn try_insert(&mut self, k: RefHalf<K>, v: RefHalf<V>) -> Result<(), (RefHalf<K>, RefHalf<V>)> {
        if self.map.contains_key(&k) {
            Err((k, v))
        } else {
            self.insert(k, v);
            Ok(())
        }
    }

    fn clone_into<RMap>(&self) -> BiMap<Self, RMap>
    where
        RMap: InnerMap<Key = Self::Value, Value = Self::Key>,
        K: Clone,
        V: Clone,
    {
        self.map
            .iter()
            .map(|(k, v)| ((**k).clone(), (**v).clone()))
            .collect()
    }
}

impl<K, V> FromIterator<(RefHalf<K>, RefHalf<V>)> for InnerHashMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (RefHalf<K>, RefHalf<V>)>,
    {
        let mut bimap = Self::default();
        for (l, r) in iter {
            bimap.insert(l, r);
        }
        bimap
    }
}
