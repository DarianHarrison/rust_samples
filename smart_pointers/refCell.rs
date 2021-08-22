#![allow(warnings)]


// RefCell<T> and the Interior Mutability Pattern
// To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. 

// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.
// The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.


// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. 
//     -   At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
//     -   References must always be valid.


// Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program; in this way, it’s conservative. 
// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context. 



// A common way to use RefCell<T> is in combination with Rc<T>. 
// Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data. 
// If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!




#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() { 

    // //Using Rc<RefCell<i32>> to create a List that we can mutate
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}


// This technique is pretty neat! By using RefCell<T>, we have an outwardly immutable List value. 
// But we can use the methods on RefCell<T> that provide access to its interior mutability so we can modify our data when we need to. 
// The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures.