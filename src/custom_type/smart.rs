use std::cell::Cell;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug)]
struct MyData<'a, T: Copy> {
    value: Cell<&'a T>
}

#[derive(Debug)]
struct SmartBox<'a, T:Copy>(&'a MyData<'a, T>);

impl<'a, T: Copy> SmartBox<'a, T> {
    fn new(data: &'a MyData<'a, T>) -> Self {
        Self(&data)
    }
}

impl<'a, T:Copy> Deref for SmartBox<'a, T> {
    type Target = Cell<&'a T>;
    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

// impl<'a, T:Copy> DerefMut for SmartBox<'a, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0.value
//     }
// }

fn main() {

    let mut data = MyData { value: Cell::new(&10) };
    let mut s1 = SmartBox::new(&data);
    println!("{:?}", (*s1).get());
    (*s1).set(&20);
    println!("{:?}", (*s1).get());
}