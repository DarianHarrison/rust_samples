#![allow(warnings)]

// Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). 
// Preventing memory leaks entirely is not one of Rust’s guarantees in the same way that disallowing data races at compile time is, meaning memory leaks are memory safe in Rust. 
// We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. 
// This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack.

    // println!("a next item = {:?}", a.tail());
}


// The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b. 
// At the end of main, Rust drops the variable b, which decreases the reference count of the Rc<List> instance from 2 to 1. 
// The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0.
// Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well.
// This instance's memory can’t be dropped either, because the other Rc<List> instance still refers to it. 
// The memory allocated to the list will remain uncollected forever.