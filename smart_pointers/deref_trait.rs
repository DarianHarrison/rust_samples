#![allow(warnings)]
use std::ops::Deref;

// Implementing the Deref trait allows you to customize the behavior of the dereference operator, * (as opposed to the multiplication or glob operator). 
// By implementing Deref in such a way that a smart pointer can be treated like a regular reference, 
// you can write code that operates on references and use that code with smart pointers too.


// Defining your own smart operator
struct MyBox<T>(T); // We define a struct named MyBox and declare a generic parameter T, because we want our type to hold values of any type. 

impl<T> MyBox<T> { 
    fn new(x: T) -> MyBox<T> { // MyBox::new function takes one parameter of type T 
        MyBox(x) // and returns a MyBox instance that holds the value passed in.
    }
}

// Implementing Deref on MyBox<T>
impl<T> Deref for MyBox<T> { // To enable dereferencing with the * operator, we implement the Deref trait.
    type Target = T; // The type Target = T; syntax defines an associated type for the Deref trait to use. 

    fn deref(&self) -> &Self::Target { // fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator. 
        &self.0
    }
}




// Implicit Deref Coercions with Functions and Methods
// Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *. 
// The deref coercion feature also lets us write more code that can work for either references or smart pointers

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    // Following the Pointer to the Value with the Dereference Operator
    assert_eq!(5, *y); // The main function that calls * on the MyBox<T> value now compiles, and the assertions pass!
    // *y behind the scenes Rust actually ran this code: *(y.deref())
    //Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method.


    // Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // here, the argument &m is a reference to a MyBox<String> value. 
    // Because we implemented the Deref trait on MyBox<T>
    // Rust can turn &MyBox<String> into &String by calling deref.
    // the standard library provides an implementation of Deref on String that returns a string slice.
    Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.

}

// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.
// Rust does deref coercion when it finds types and trait implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>


