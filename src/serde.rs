use crate::{
    bimap::RawBiMap,
    traits::{IterateRef, Map, MapKind},
    traits::Insert,
};

use core::{fmt, marker::PhantomData};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<L, R, LMap, RMap> Serialize for RawBiMap<LMap, RMap>
where
    L: Serialize,
    R: Serialize,
    LMap: Map<Key = L, Value = R> + for<'a> IterateRef<'a>,
    RMap: Map<Key = R, Value = L>,
{
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.collect_map(self.iter_left())
    }
}

struct BiMapVisitor<L, R, LKind, RKind> {
    marker: PhantomData<(L, R, LKind, RKind)>,
}

impl<'de, L, R, LKind, RKind> Visitor<'de> for BiMapVisitor<L, R, LKind, RKind>
where
    L: Deserialize<'de>,
    R: Deserialize<'de>,
    LKind: MapKind<L, R>,
    RKind: MapKind<R, L>,
{
    type Value = RawBiMap<L, R>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut entries: A) -> Result<Self::Value, A::Error> {
        // while let Some((l, r)) = entries.next_entry()? {
        //     map.insert(l, r);
        // }
        // Ok(map)
        todo!()
    }
}
