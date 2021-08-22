
source: https://doc.rust-lang.org/book/title-page.html

# 1. Getting Started

a) Install
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
restart shell
```
rustup update
rustc --version
```

b) cargo
```
cargo --version
cargo new hello_cargo
cd hello_cargo

cat << 'EOF' > Cargo.toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
EOF
```
```
cargo check # run cargo check periodically to make sure it compiles, (does not compile code)
cargo update # update pachages in Cargo.toml
cargo run # to compile and run
```
Production Build
```
cargo build --release # compile it with optimizations
```

# 2. Guessing Game
```
cd 2.guessing_game
cargo run # to compile and run
```

# 3. Common Programming Concepts

variables, basic data types, basic functions and basic loops

# 4. Ownership

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. 

# 5. Structs

A struct is like an object’s data attributes

```
cd 5.structs

rustc -A warnings 1.structs_basics.rs
rustc -A warnings 2.structs_example.rs
rustc -A warnings 3.method_syntax.rs
```

# 6. Enums and Pattern Matching

A better alternative of struct. We can represent concepts in a more concise manner by putting data directly into each enum variant. ipv4 ipv6 example

# 7. Managing Growing Projects with Packages, Crates, and Modules

the "use" keyword  brings a path into scope; 
the "pub" keyword makes items public. 
the "as" keyword, 
external packages/dependencies, 
and the glob operator.

# 8. Common Collections

A vector allows you to store a variable number of values next to each other.
A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

Note that String and Vec<T> are considered smart pointers

# 9. Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code-plus one or more of the following:

- The bad state is not something that’s expected to happen occasionally.
- Your code after this point needs to rely on not being in this bad state.
- There’s not a good way to encode this information in the types you use.


If someone calls your code and passes in values that don’t make sense, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.


# 10. Generic Types, Traits, and Lifetimes

Generic type parameters let you apply the code to different types. 
Traits(interfaces) and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. 
lifetime annotations help ensure that our flexible code won’t have any dangling references.
And all of this analysis happens at compile time, which doesn’t affect runtime performance!

with generic type parameters, traits and trait bounds, and generic lifetime parameters, we can write code without repetition that works in many different situations.


# 11. Writing Automated Tests

```
cd automated_tests

cargo test
cargo test -- --test-threads=1 # with multiple threads
cargo test it_works # run only one test

cargo test -- --ignored # run tests that are marked as ignored (maybe it takes an hour to run)
```

# 12. An I/O Project: Building a Command Line Program

```
cargo test
cargo run to poem.txt > output.txt
```

# 13. Functional Language Features: Iterators and Closures

Closures, a function-like construct you can store in a variable
Iterators, a way of processing a series of elements

// The iterator version was slightly faster than for loop

Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.

# 14. More about Cargo and Crates.io

```
cool documentatio feature
```

# 15. Smart Pointers

a variable that contains an address in memory. This address refers to, or “points at,” some other data, the most common kind of pointer in Rust is a reference.

Smart pointers, on the other hand, are data structures that also have additional metadata and capabilities. 
* The Box<T> type has a known size and points to data allocated on the heap
* The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners. Allows you to share data between multiple parts of your program for reading only. 
* The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.
* Deref : implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.
* Drop traits: lets you customize what happens when a value is about to go out of scope.


Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

# 16. Fearless Concurrency

```


```

# 17. Object Oriented Programming Features of Rust

```


```

# 18. Patterns and Matching

```


```

# 19. Advanced Features

```


```

# 20. Final Project: Building a Multithreaded Web Server


Best Ideas thus far:

Traits as components to ML
Iterators are faster than loops
Enums are beetter than structs