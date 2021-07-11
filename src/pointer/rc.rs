use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use super::cell::Cell;

#[derive(Debug)]
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    fn new(value: T) -> Rc<T> {
        let inner = RcInner {
            value,
            refcount: Cell::new(1),
        };
        let inner = Box::new(inner);
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.inner.as_ref() };
        &inner.value
    }
}

fn print_ref_copy<T>(copied_ref: &RcInner<T>) {
    println!("copied ref: {:p}", &copied_ref);
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        if count == 1 {
            // This is showed in video, but it doesn't prevent inner is used later and it doesn't cause panic.
            // It is okay because inner as a reference is copied to drop call,
            // drop can be regarded as a normal function, which can be verified by print_ref_copy.
            // it's better to let inner = ();
            println!("original inner ref: {:p}", &inner);
            print_ref_copy(inner);

            drop(inner);

            println!("original inner ref: {:p}", &inner);

            // ref1 == ref2, which is inner address
            println!("inner ref1 before drop: {:p}", inner);
            println!("inner ref2 before drop: {:p}", unsafe {
                self.inner.as_ref()
            });
            {
                unsafe { Box::from_raw(self.inner.as_ptr()) };
            }

            // "dropping RcInner" is printed before this
            println!("after drop box inner");

            // Inner can still be set and get after Box::from_raw.
            // But ptr::drop_in_place can't be called anymore:
            // malloc: *** error for object 0x7fedbb5043f0: pointer being freed was not allocated
            // malloc: *** set a breakpoint in malloc_error_break to debug
            // unsafe { ptr::drop_in_place(self.inner.as_ptr()) };
            //
            // https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8#gistcomment-3807015
            // It's because freeing memory just makes it available for future allocations,
            // it doesn't actually make the memory inaccessible. The memory is still there,
            // it's just undefined behavior to access it.
            inner.refcount.set(100);
            println!("inner ref after drop: {:p}", unsafe { self.inner.as_ref() });
            println!(
                "now refcount: {:?}",
                unsafe { self.inner.as_ref() }.refcount.get()
            );
        } else {
            println!("count-1");
            inner.refcount.set(count - 1);
        }
    }
}

impl<T> Drop for RcInner<T> {
    fn drop(&mut self) {
        println!("dropping RcInner");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let rc = Rc::new(5);
        assert_eq!(*rc, 5);
    }

    #[test]
    fn clone() {
        let rc1 = Rc::new("abc".to_owned());
        let rc2 = rc1.clone();
        let addr1 = format!("{:p}", rc1.inner);
        let addr2 = format!("{:p}", rc2.inner);
        assert_eq!(addr1, addr2);
        assert_eq!(*rc1, "abc");
        assert_eq!(*rc2, "abc");
        drop(rc2);
        // let ptr = unsafe { rc1.inner.as_ref().unwrap() };
        drop(rc1);
        // Don't know why this can still work, maybe as_ref will track the data and doesn't release it
        // until now
        // println!("{:?}", ptr.value);
    }
}
