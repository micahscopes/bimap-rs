use alloc::boxed::Box;
use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    ops::Deref,
};

pub struct RefPair<T>(pub SemiRef<T>, pub SemiRef<T>);

impl<T> RefPair<T> {
    pub fn split(value: T) -> Self {
        let ptr: *const T = Box::into_raw(Box::new(value));
        Self(SemiRef { ptr }, SemiRef { ptr })
    }

    pub fn join(self) -> T {
        let Self(a, b) = self;
        assert!(core::ptr::eq(a.ptr, b.ptr));
        unsafe { *Box::from_raw(a.ptr as *mut T) }
    }
}

pub struct SemiRef<T> {
    ptr: *const T,
}

impl<T> Deref for SemiRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> Debug for SemiRef<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T> Hash for SemiRef<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self, state)
    }
}

impl<T> Eq for SemiRef<T> where T: Eq {}

impl<T> Ord for SemiRef<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        T::cmp(self, other)
    }
}

impl<T> PartialEq for SemiRef<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        T::eq(self, other)
    }
}

impl<T> PartialOrd for SemiRef<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        T::partial_cmp(self, other)
    }
}

impl<K, Q: ?Sized> Borrow<Wrapped<Q>> for SemiRef<K>
where
    K: Borrow<Q>,
{
    fn borrow(&self) -> &Wrapped<Q> {
        K::borrow(self).wrap()
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
