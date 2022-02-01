use std::{cell::Cell, ops::Deref};
use std::ptr::NonNull;

// use std::{rc::Rc, cell::RefCell, borrow::BorrowMut,};

#[derive(Debug)]
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>
}

#[derive(Debug)]
struct Rc<T> {
    inner: *mut RcInner<T>,
}

impl<T> Rc<T> {
    fn new(value: T) -> Self {

        let inner = Box::new(RcInner { value, refcount: Cell::new(1) });
        Self {
            inner: Box::into_raw(inner)
        }
    }

    fn strong_count(rc: &Rc<T>) -> usize {
        (unsafe { &*rc.inner }).refcount.get()
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner }; 
        let count = inner.refcount.get();
        inner.refcount.set(count + 1);
        Rc { inner: self.inner }
    }
}
impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner).value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { &*self.inner }; 
        let count = inner.refcount.get();
        
        if count == 1 {
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            inner.refcount.set(count - 1);
        }
    }
}
fn main() {
    
    let rc= Rc::new(10);
    println!("{:?}", *rc);
    println!("Count {:?}", Rc::strong_count(&rc));

    let rc2 = Rc::clone(&rc);
    println!("Count {:?}", Rc::strong_count(&rc2));

}
// enum List {
//     Cons(i32, Rc<List>),
//     Nil
// }
// fn main() {

//     let a = Rc::new(List::Cons(10, Rc::new(List::Cons(20, Rc::new(List::Nil)))));

//     let b = List::Cons(5, Rc::clone(&a));
//     println!("count {}", Rc::strong_count(&a));

//     let c = List::Cons(2, Rc::clone(&a));
//     println!("count {}", Rc::strong_count(&a));

//     let a = Rc::new(10);
//     let b = Rc::clone(&a);

//     let mut immt = Box::new(10);

//     println!("{}", immt);
//     it_sends_an_over_75_percent_warning_message();
// }

// struct MockMessenger {
//     sent_messages: Vec<String>,
// }

// impl MockMessenger {
//     fn new() -> MockMessenger {
//         MockMessenger {
//             sent_messages: vec![],
//         }
//     }
// }

// impl Messenger for MockMessenger {
//     fn send(&mut self, message: &str) {
//         self.sent_messages.push(String::from(message));
//     }
// }

// fn it_sends_an_over_75_percent_warning_message() {
//     let mut mock_messenger = MockMessenger::new();
//     let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);

//     limit_tracker.set_value(80);

//     assert_eq!(mock_messenger.sent_messages.len(), 1);
// }

// pub trait Messenger {
//     fn send(&mut self, msg: &str);
// }

// pub struct LimitTracker<'a, T: Messenger> {
//     messenger: &'a mut T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &mut T, max: usize) -> LimitTracker<T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }
