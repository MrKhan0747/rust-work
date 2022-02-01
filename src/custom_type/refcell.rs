use std::{cell::{UnsafeCell, Cell}, ops::{Deref, DerefMut}};

#[derive(Copy, Clone)]
enum RefCellState {
    Shared(isize),
    Exclusive,
    Unshared
}

struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefCellState>
}

impl<T> RefCell<T> {
    
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefCellState::Unshared)
        }
    }

    fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Shared(1));
                
                Some(Ref{ refcell: self })
            },
            RefCellState::Shared(count) => {
                self.state.set(RefCellState::Shared(count + 1));
                Some(Ref{ refcell: self })
            },
            RefCellState::Exclusive => None
        }
    }

    fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Exclusive);
                Some(RefMut{ refcell: self })
            },
            RefCellState::Shared(_) | RefCellState::Exclusive => {
                None
            }
        }
    }
}

pub struct Ref<'r, T> {
    refcell: &'r RefCell<T>
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
        
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Exclusive | RefCellState::Unshared => { unimplemented!() },
            RefCellState::Shared(1) => {
                self.refcell.state.set(RefCellState::Unshared)
            },
            RefCellState::Shared(n) => {
                self.refcell.state.set(RefCellState::Shared(n - 1))
            }
        }
    }
}

pub struct RefMut<'r, T> {
    refcell: &'r RefCell<T>
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }   
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Shared(_) | RefCellState::Unshared => { unimplemented!() },
            RefCellState::Exclusive=> {
                self.refcell.state.set(RefCellState::Unshared)
            },
        }
    }    
}

fn main() {

}