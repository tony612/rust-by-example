use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T>

impl<T> Cell<T> {
    pub fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, val: T) {
        // SAFETY:
        // No concurrency problem because !Sync, no others are mutating self.value
        // No null pointer problem because we don't give the ownership to others
        unsafe {
            *self.value.get() = val;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY:
        // see set
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct Foo {
        #[allow(dead_code)]
        a: usize,
        b: Cell<usize>,
    }

    #[test]
    fn works() {
        let foo = Foo {
            a: 1,
            b: Cell::new(1),
        };

        // compile error
        // foo.a = 2;
        foo.b.set(2);
        assert_eq!(foo.b.get(), 2);
    }
}
