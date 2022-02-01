use std::cell::UnsafeCell;

struct Cell<T> {
    value: UnsafeCell<T>
}

impl<T> Cell<T> {

    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value)
        }
    }
    fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }
}

impl<T: Copy> Cell<T> {

    fn get(&self) -> T {
        unsafe { *self.value.get() }
    }    
}

fn main() {

    let mut c1 =  Cell::new(10);
    println!("{}", c1.get());
    c1.set(20);
    println!("{}", c1.get());
}