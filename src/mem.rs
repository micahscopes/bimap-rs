use alloc::rc::Rc;
use core::{borrow::Borrow, ops::Deref};

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

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ref<T> {
    rc: Rc<T>,
}

impl<T> Ref<T> {
    pub fn share(x: T) -> (Ref<T>, Ref<T>) {
        let rc = Rc::new(x);
        (Ref { rc: rc.clone() }, Ref { rc })
    }

    pub fn rejoin(a: Ref<T>, b: Ref<T>) -> T {
        assert!(Rc::ptr_eq(&a.rc, &b.rc));
        core::mem::drop(b);
        Rc::try_unwrap(a.rc).ok().unwrap()
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.rc
    }
}

impl<K, Q: ?Sized> Borrow<Wrapped<Q>> for Ref<K>
where
    K: Borrow<Q>,
{
    fn borrow(&self) -> &Wrapped<Q> {
        K::borrow(self).wrap()
    }
}
