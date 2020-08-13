use alloc::boxed::Box;
use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    ops::Deref,
};

pub struct Semi<T> {
    ptr: *const T,
}

impl<T> Semi<T> {
    pub fn share(value: T) -> [Self; 2] {
        let ptr: *const T = Box::into_raw(Box::new(value));
        [Self { ptr }, Self { ptr }]
    }

    pub fn reunite(parts: [Self; 2]) -> T {
        let [a, b] = parts;
        assert!(core::ptr::eq(a.ptr, b.ptr));
        unsafe { *Box::from_raw(a.ptr as *mut T) }
    }
}

impl<T> Deref for Semi<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<K, Q: ?Sized> Borrow<Wrapped<Q>> for Semi<K>
where
    K: Borrow<Q>,
{
    fn borrow(&self) -> &Wrapped<Q> {
        K::borrow(self).wrap()
    }
}

impl<T> Debug for Semi<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T> Hash for Semi<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self, state)
    }
}

impl<T> Eq for Semi<T> where T: Eq {}

impl<T> Ord for Semi<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        T::cmp(self, other)
    }
}

impl<T> PartialEq for Semi<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        T::eq(self, other)
    }
}

impl<T> PartialOrd for Semi<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        T::partial_cmp(self, other)
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Wrapped<T: ?Sized>(T);

pub trait Wrap {
    fn wrap(&self) -> &Wrapped<Self>;
}

impl<T: ?Sized> Wrap for T {
    fn wrap(&self) -> &Wrapped<T> {
        let ptr = self as *const T as *const Wrapped<T>;
        unsafe { &*ptr }
    }
}
