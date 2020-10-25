use alloc::boxed::Box;
use core::{cell::Cell, cmp, fmt, hash, ops::Deref, ptr::NonNull};

struct Inner<T> {
    value: T,
    safe_to_drop: Cell<bool>,
}

impl<T> Inner<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            safe_to_drop: Cell::new(false),
        }
    }
}

pub struct Half<T> {
    ptr: NonNull<Inner<T>>,
}

impl<T> Half<T> {
    pub fn halve(value: T) -> [Self; 2] {
        let ptr: NonNull<_> = Box::leak(Box::new(Inner::new(value))).into();
        [Self { ptr }, Self { ptr }]
    }

    pub fn rejoin(a: Self, b: Self) -> T {
        assert!(core::ptr::eq(a.ptr.as_ptr(), b.ptr.as_ptr()));
        let inner = unsafe { Box::from_raw(a.ptr.as_ptr()) };
        core::mem::forget(a);
        core::mem::forget(b);
        inner.value
    }
}

impl<T> Drop for Half<T> {
    fn drop(&mut self) {
        unsafe {
            if self.ptr.as_ref().safe_to_drop.replace(true) {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Deref for Half<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T: fmt::Debug> fmt::Debug for Half<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

impl<T: Eq> Eq for Half<T> {}

impl<T: hash::Hash> hash::Hash for Half<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        T::hash(self, state);
    }
}

impl<T: Ord> Ord for Half<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        T::cmp(self, other)
    }
}

impl<T: PartialEq> PartialEq for Half<T> {
    fn eq(&self, other: &Self) -> bool {
        T::eq(self, other)
    }
}

impl<T: PartialOrd> PartialOrd for Half<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        T::partial_cmp(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct DropCount<'a> {
        counter: &'a Cell<usize>,
    }

    impl<'a> Drop for DropCount<'a> {
        fn drop(&mut self) {
            self.counter.set(self.counter.get() + 1);
        }
    }

    #[test]
    fn sequential_drop() {
        let counter = Cell::new(0);
        let [a, b] = Half::halve(DropCount { counter: &counter });
        assert_eq!(counter.get(), 0);
        drop(a);
        assert_eq!(counter.get(), 0);
        drop(b);
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn reunite_pass() {
        let counter = Cell::new(0);
        let [a, b] = Half::halve(DropCount { counter: &counter });
        assert_eq!(counter.get(), 0);
        let original = Half::rejoin(a, b);
        assert_eq!(counter.get(), 0);
        drop(original);
        assert_eq!(counter.get(), 1);
    }

    #[test]
    #[should_panic]
    fn reunite_fail() {
        let counter = Cell::new(0);
        let [a, _] = Half::halve(DropCount { counter: &counter });
        let [_, d] = Half::halve(DropCount { counter: &counter });
        assert_eq!(a, d);
        let _ = Half::rejoin(a, d);
    }

    #[test]
    #[should_panic]
    fn reunite_fail2() {
        let [a, _] = Half::halve(());
        let [_, d] = Half::halve(());
        assert_eq!(a, d);
        let _ = Half::rejoin(a, d);
    }
}
