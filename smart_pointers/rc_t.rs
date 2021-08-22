#![allow(warnings)]


// To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting.
// The Rc<T> type keeps track of the number of references to a value which determines whether or not a value is still in use.
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

// Rc<T> allows you to share data between multiple parts of your program for reading only. 

// Similar to Rc<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context. 

// A definition of List that uses Rc<T>
enum List {
    Cons(i32, Rc<List>), // 
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc; // We need to add a use statement to bring Rc<T> into scope because it’s not in the prelude. 

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // we create the list holding 5 and 10 and store it in a new Rc<List> in a
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Then when we create b and c.
    // Cloning an Rc<T> Increases the Reference Count
    let b = Cons(3, Rc::clone(&a)); // we call the Rc::clone function and pass a reference to the Rc<List> in a as an argument.
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // we call the Rc::clone function and pass a reference to the Rc<List> in a as an argument.
        println!("count after creating c = {}", Rc::strong_count(&a));
        // the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}