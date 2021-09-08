# Quick Reference

## References, Mutable References, and Borrowing:

* The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
* we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length.
* How to modify something we’re borrowing ? We can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
fn calculate_length(s: &String) -> usize { // only read borrowing
    s.len()
}

fn change(s: &mut String) { // mutable borrowing
    s.push_str(", world");
}

fn main() {

    let mut g_list = String::from("hello");
    println!("s = {}", g_list);

    let len = calculate_length(&g_list);
    println!("The length of '{}' is {}.", g_list, len);

    {
        change(&mut g_list);
	    println!("s = {}", g_list);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    change(&mut g_list);
    println!("s = {}", s);
}
```

## OOP:

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

## Expressions:
evaluate to something, Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}.
```rust
let y = {
    let x = 3;
    x + 1
};
```
4


## Iterators:
Using iterator adaptor methods in the implementation of the search function
```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();

    println!("{:?}",counter.next());
    println!("{:?}",counter.next());
    println!("{:?}",counter.next());

    
}

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}",sum);
}

fn main(){

calling_next_directly();
using_other_iterator_trait_methods();
}
```
Some(1)
Some(2)
Some(3)
18


## Shadowing:
```rust
let x = x + 1;
let x = x * 2;
println!("The value of x is: {}", x)
```

## isize and usize: 
types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit archit
You should use usize whenever you deal with something related to container size, and u32 and u64 for everything else

## Arrays: 
have a fixed length and type, may be good for iterating
```rust
let a = [3; 5];
same as:  let a = [3, 3, 3, 3, 3];
```

## Pseudo Ternary Opereator / Control FLow
```rust
let i = 10
let y_order = 3
let mut a: Vec<usize> = if (i < y_order) { vec![y_order[i]] } else { Vec::new() };
```


## Stack-Only Data: Copy
```rust
let x = 5;
let y = x;
```

## Ownership: fucntions, return values, and Scope:
```rust
fn main() {

    let s = String::from("hello");      // s comes into scope

    takes_ownership(s);                 // s's value moves into the function ... and so is no longer valid here

    let x = 5;                          // x comes into scope

    makes_copy(x);                      // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3

}   // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
    // s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.


fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string                              // some_string is returned and moves out to the calling function
}


// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}


fn takes_ownership(some_string: String) {       // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.


fn makes_copy(some_integer: i32) {              // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```


## Structs & Methods:

```rust
// define
struct User { 
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn print_user(&self) -> bool{
        println!("ueser={}, email={}",self.username,self.email);
        true
    }
}

// instantiate a mutable struct

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
let a = user1.print_user();
```

## Modules, Crates, and Project Structure:
A logical group of code is called a Module. Multiple modules are compiled into a unit called crate.

* module: Logically groups code within a crate.
* crate: Is a compilation unit in Rust; Crate is compiled to binary or library.
* cargo: The official Rust package management tool for crates.


```
.
├── Cargo.lock
├── Cargo.toml
├── examples
│   ├── load_data.rs
│   └── pre-process.rs
├── src
│   ├── main.rs
│   ├── utils
│   │   ├── skf.rs
│   │   └── mlp.rs
│   └── utils.rs
└── tests
```

src/utils.rs
```rust
pub mod skf;
pub mod mlp;
```

src/main.rs
```rust
mod utils;
use utils::sfk;

fn main(){
}
```


## Closures:
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.

```rust
use std::thread;
use std::time::Duration;

fn main() {

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

    println!("Today, do {} pushups!", expensive_closure(7));
}
```

## Match & Enum:
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let a = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value {}!", a);
}
```
State quarter from Alaska!
Value 25!

## Features:

in Cargo.toml
```rust
[dependencies]
ndarray = "0.15"
rand = { version = "0.8.4", features = ["std_rng"] }
```

## Template Strings
```rust
let name = world;
format!("Hello {}!", name);
```

## TODO: Test Driven Development
test driven development

## Design Patterns and Best Practices
```
https://rust-unofficial.github.io/patterns/intro.html
```
TLDR; 

### 1) Idioms

a) Use borrowed types for arguments
* you should always prefer using the **borrowed** type over **borrowing the owned type**. Such as &str over &String, &[T] over &Vec<T>, or &T over &Box<T>.

b) Temporary mutability
* Using nested block:
```rust
let data = {
    let mut data = get_vec();
    data.sort();
    data
};

// Here `data` is immutable.
```
* Using variable rebinding:
```rust
let mut data = get_vec();
data.sort();
let data = data;

// Here `data` is immutable.
```

### 2) Design Patterns

a) Command: 
* The basic idea of the Command pattern is to separate out actions into its own objects and pass them as parameters.
* Example: Suppose we have a sequence of actions or transactions encapsulated as objects. We want these actions or commands to be executed or invoked in some order later at different time. These commands may also be triggered as a result of some event. For example, when a user pushes a button, or on arrival of a data packet
```
https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html
```

b) Resource Initialization and Finalization
* The essence of the pattern is that resource initialisation is done in the constructor of an object and finalisation in the destructor.

c) Visitor
* A visitor encapsulates an algorithm that operates over a heterogeneous collection of objects. It allows multiple different algorithms to be written over the same data without having to modify the data. Furthermore, the visitor pattern allows separating the traversal of a collection of objects from the operations performed on each object.
*  If data is homogeneous, you can use an iterator-like pattern.
* Using a visitor object (rather than a functional approach) allows the visitor to be stateful and thus communicate information between nodes.
* The visitor pattern is closely related to fold. They share the concept of walking a data structure performing an operation on each node. The visitor does not create a new data structure nor consume the old one.

d) Fold (similar to map)
* Run an algorithm over each item in a collection of data to create a new item, thus creating a whole new collection.
* Like the visitor pattern, the fold pattern allows us to separate traversal of a data structure from the operations performed to each node.
* Using a reference counted pointer gives the best of both worlds - we can reuse the original data structure, and we don't need to clone unchanged nodes. However, they are less ergonomic to use and mean that the data structures cannot be mutable.
* The visitor pattern is closely related to fold. They share the concept of walking a data structure performing an operation on each node. The visitor does not create a new data structure nor consume the old one.

e) Decouple code where possible
f) Prefer small crates that do one thing well. ( encourage more modular code )
g) Contain unsafety in small modules
* If you have unsafe code, create the smallest possible module that can uphold the needed invariants to build a minimal safe interface upon the unsafety. 

### 3) Anti-Patterns

a) Clone to satisfy the borrow checker

* The borrow checker prevents Rust users from developing otherwise unsafe code by ensuring that either: 
1) only one mutable reference exists, or 
2) potentially many but all immutable references exist. 

* Using .clone() causes a copy of the data to be made. Any changes between the two are not synchronized -- as if two completely separate variables exist
* In general, clones should be deliberate, with full understanding of the consequences. If a clone is used to make a borrow checker error disappear, that's a good indication this anti-pattern may be in use.

b) leave the warnings flag in the code
* remember to remove any ```#[deny(warnings)]```  because we actually want to satisfy all warnings for higher quality code

c) Deref polymorphism
* Try not to abuse the Deref trait to emulate inheritance between structs, and thus reuse methods.
* The Deref trait is designed for the implementation of custom pointer types. The intention is that it will take a pointer-to-T to a T, not convert between different types.

d) FUnctional rather than Imperative
* choose declarative (functional) approach
```rust
println!("{}", (1..11).fold(0, |a, b| a + b));
```
* instead of imperative approach
```rust
let mut sum = 0;
for i in 1..11 {
    sum += i;
}
println!("{}", sum);
```