#![allow(warnings)]


// Using Box<T> to Point to Data on the Heap

// You’ll use them most often in these situations:

// When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type



// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. 
// This means we can put a Box<T> inside the Cons variant instead of another List value directly. 
// The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant. 
// Conceptually, we still have a list, created with lists “holding” other lists, but this implementation is now more like placing the items next to one another rather than inside one another.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let b = Box::new(5); // Storing an i32 value on the heap using a box
    println!("b = {}", b);

    // At compile time, Rust needs to know how much space a type takes up. 
    // One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. 
    // Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. 
    // However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.


    // Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use. 
    // Other, more complex recursive data types are useful in various situations, but by starting with the cons list, we can explore how boxes let us define a recursive data type without much distraction.

	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}


// To determine how much space to allocate for a Message value, 
// Rust goes through each of the variants to see which variant needs the most space. 
// Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, and so forth. 
// Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


