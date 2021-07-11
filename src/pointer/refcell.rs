use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use super::cell::Cell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> RefCell<T> {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&self) -> Option<Ref<T>>
    where
        T: Copy,
    {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(count) => {
                self.state.set(RefState::Shared(count + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    fn borrow_mut(&self) -> Option<RefMut<T>>
    where
        T: Copy,
    {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<'a, T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // Only read ref is get, it's safe to borrow more read borrow
        unsafe { &*self.refcell.value.get() }
    }
}

impl<'a, T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(count) => self.refcell.state.set(RefState::Shared(count - 1)),
        }
    }
}

struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // No ref/mut_ref is borrowed, it's safe to borrow exclusive value
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        // No ref/mut_ref is borrowed, it's safe to borrow exclusive value
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<'a, T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn borrow() {
        let c = RefCell::new(5);

        let borrowed_five = c.borrow().unwrap();
        let borrowed_five2 = c.borrow().unwrap();
        assert_eq!(*borrowed_five, 5);
        assert_eq!(*borrowed_five2, 5);
    }

    #[test]
    fn borrow_after_mut() {
        let c = RefCell::new(5);

        let mut borrowed_five = c.borrow_mut().unwrap();
        *borrowed_five = 6;

        assert!(c.borrow().is_none());
        assert!(c.borrow_mut().is_none());
    }

    #[test]
    fn mut_after_borrow() {
        let c = RefCell::new(5);

        let borrowed_five = c.borrow().unwrap();
        assert_eq!(*borrowed_five, 5);
        // Shared, borrow_mut
        assert!(c.borrow_mut().is_none());
    }

    #[test]
    fn mut_after_one_borrow_drop() {
        let c = RefCell::new(5);

        // Shared(1)
        let borrowed1 = c.borrow().unwrap();
        // Shared(2)
        let borrowed2 = c.borrow().unwrap();
        assert_eq!(*borrowed1, 5);
        assert_eq!(*borrowed2, 5);
        // Shared(1)
        drop(borrowed1);

        let borrowed_mut = c.borrow_mut();
        assert!(borrowed_mut.is_none());
    }

    #[test]
    fn mut_after_borrow_drop() {
        let c = RefCell::new(5);

        // Shared(1)
        let borrowed1 = c.borrow().unwrap();
        // Shared(2)
        let borrowed2 = c.borrow().unwrap();
        assert_eq!(*borrowed1, 5);
        assert_eq!(*borrowed2, 5);
        // Shared(1)
        drop(borrowed1);
        // Unshared
        drop(borrowed2);

        let mut borrowed_mut = c.borrow_mut().unwrap();
        assert_eq!(*borrowed_mut, 5);
        *borrowed_mut = 6;
    }

    #[test]
    fn borrow_after_mut_drop() {
        let c = RefCell::new(5);

        let mut borrowed_mut = c.borrow_mut().unwrap();
        assert_eq!(*borrowed_mut, 5);
        *borrowed_mut = 6;
        drop(borrowed_mut);

        let borrowed = c.borrow().unwrap();
        assert_eq!(*borrowed, 6);
    }
}
