
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

todo: this chapter has some work to do, will fix in retrospective
notice we are quieting unused_code and other warnings just for demo's sake

```
cd 3.programming_concepts

rustc -A warnings vars_and_mutability
rustc -A warnings data_types
rustc -A warnings functions
rustc -A warnings control_flow
```

# 4. Ownership

todo: this chapter has some work to do, will fix in retrospective

```
cd 4.ownership

rustc -A warnings ownership_basics
rustc -A warnings references_borrowing
rustc -A warnings slice_type
```

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

```


```

# 7. Managing Growing Projects with Packages, Crates, and Modules

the "use" keyword  brings a path into scope; 
the "pub" keyword makes items public. 
the "as" keyword, 
external packages, 
and the glob operator.

Create a new library named restaurant by running 
```
cargo new --lib restaurant;
```
```
cat << 'EOF' > src/lib.rs

fn serve_order() {}

mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // We can construct relative paths that begin in the parent module by using super. We use super so we’ll have fewer places to update code in the future if this code gets moved to a different module.
    }

    fn cook_order() {}

    // we've defined a public back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


// In contrast, if we make an enum public, all of its variants are then public. 
mod enum_back_of_house_enum {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_enum() {
    let order1 = back_of_house_enum::Appetizer::Soup;
    let order2 = back_of_house_enum::Appetizer::Salad;
}


// Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant
mod front_of_house {
    pub mod hosting { 
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // paths are followed by one or more identifiers separated by double colons (::).

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // paths are followed by one or more identifiers separated by double colons (::).

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye"); 
    
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat"); // 
    println!("I'd like {} toast please", meal.toast);

    // The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default. 
	// Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
    
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
EOF
```
Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Bringing Paths into Scope with the use Keyword
```
src/lib.rs

use std::collections::HashMap; // Bringing HashMap into scope in an idiomatic way
use std::io::Result as IoResult; // Providing New Names with the as Keyword
use std::fmt::Result;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
 
// use crate::front_of_house::hosting; // Bringing a module into scope with use
use self::front_of_house::hosting; // Bringing a module into scope with use and relative path

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
Using External Packages, Cargo.toml

```
[dependencies]
rand = "0.8.3"
```
```
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```
make imports shorter
```
// --snip--
use std::{cmp::Ordering, io};
// --snip--
use std::io::{self, Write};

use std::collections::*; // bring in all public items
```
Separating Modules into Different Files
Filename: src/lib.rs
```
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
Filename: src/front_of_house.rs
```
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```


# 8. Common Collections

A vector allows you to store a variable number of values next to each other.
A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

```


```

# 9. Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code-plus one or more of the following:

- The bad state is not something that’s expected to happen occasionally.
- Your code after this point needs to rely on not being in this bad state.
- There’s not a good way to encode this information in the types you use.


If someone calls your code and passes in values that don’t make sense, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.

```


```

# 10. Generic Types, Traits, and Lifetimes

- Try to decouple duplicated code where possible and put it into a function:

```
cd 5.structs


```

# 11. Writing Automated Tests

```


```

# 12. An I/O Project: Building a Command Line Program

```


```

# 13. Functional Language Features: Iterators and Closures

```


```

# 14. More about Cargo and Crates.io

```


```

# 15. Smart Pointers

```


```

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