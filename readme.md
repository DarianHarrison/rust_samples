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

## Generic Data Types
We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
```Rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```
p3.x = 5, p3.y = c

## Traits
Define Shared Behavior. A trait tells the Rust compiler about functionality a particular type has and can share with other types.

1. Implementing a trait on a type:
* impl trait-name for type-name 
* Within the impl block, we put the method signatures that the trait definition has defined. Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
1 new tweet: horse_ebooks: of course, as you probably already know, people


2. One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate. This restriction is part of a property of programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa.

3. Defaults Implementations
```Rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Definition of a Summary trait with a default implementation of the summarize method
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

}
```
Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.


4. Traits as Parameters: how to use traits to define functions that accept many different types.

* We can define a **notify function** that calls the **summarize method** on its **item parameter**, which is of some **type that implements** the **Summary trait**.
```Rust
// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
// This parameter accepts any type that implements the specified trait.
pub fn notify(item: &impl Summary) {
    // we can call any methods on item that come from the Summary trait, such as summarize
    println!("Breaking news! {}", item.summarize());
}
```
note: Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.


* Trait Bound Syntax
The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound; it looks like this:
```Rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
If we wanted this function to allow **item1** and **item2** to have different types, using impl Trait would be appropriate (as long as both types implement Summary).
```Rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```
If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:
```Rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

* Specifying Multiple Trait Bounds with the + Syntax
We can specify more than one trait bound. 
we can specify in the **notify** definition that **item** must **implement** both **Display and Summary**. We can do so using the + syntax:
```Rust
pub fn notify(item: &(impl Summary + Display)) {
```
The **+** syntax is also valid with trait bounds on generic types:
With the two trait bounds specified, the body of notify can call summarize and use {} to format item.
```Rust
pub fn notify<T: Summary + Display>(item: &T) {
```

* Clearer Trait Bounds with where Clauses
Multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. 
For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. 

you can use this
```Rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```
instead of
```Rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

* Returning Types that Implement Traits
We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn returns_summarizable() -> impl Summary { // return a type that is specified by a trait
    Tweet { // can be Tweet or Summary type
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```
Note: You can only use impl Trait if you’re returning a single type.
Note2: The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators.
Example: The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.

* Example: A working definition of the **largest function** that works on **any generic type** that **implements** the **PartialOrd** and **Copy** **traits**
```Rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

```

* Using Trait Bounds to Conditionally Implement Methods

Conditionally implement methods on a generic type depending on trait bounds

```Rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
We can also conditionally implement a trait for any type that implements another trait.
```Rust
impl<T: Display> ToString for T {
```
Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.


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

## Smart Pointers
* Smart pointers: are data structures that act like a pointer and also have additional metadata and capabilities, the most common smart pointers in the standard library are:
* Deref : implementing Deref allows smart pointer to be treated like a regular references.
* Drop traits: lets you customize what happens when a value is about to go out of scope.


1. The **Box<T>** type has a known size and points to data allocated on the heap, You’ll use them most often in these situations:
    -   When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    -   When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    -   When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
```Rust
fn main() {

    let x = 5;
    // Storing an i32 value on the heap using a box
    let y = Box::new(x);

    println!("{:?}", x);

    // Using the dereference operator to follow a reference to an i32 value
    println!("{:?}", *y);
}
```

2. The **Rc<T>** (Reference Counting) type keeps track of the number of references to data on the heap so that data can have multiple owners. Allows you to share data between multiple parts of your program for reading only. 
```Rust
// Example: Printing the reference count

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nl};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

3.  The **RefCell<T>** type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.


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

d) Functional rather than Imperative
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

e) Generics as Type Classes
* In Rust, a generic type parameter creates what is known in functional languages as a "type class constraint", and each different parameter filled in by an end user actually changes the type. In other words, Vec<isize> and Vec<char> are two different types, which are recognized as distinct by all parts of the type system.
* We can create our own custom typess
* Rust's additional methods can be type checked when they are used, because their generics are statically defined. That makes them more usable while remaining safe.


### 4) Design Principles
* SOLID
1) Single Responsibility Principle (SRP): A class should only have a single responsibility, that is, only changes to one part of the software's specification should be able to affect the specification of the class.
2) Open/Closed Principle (OCP): "Software entities ... should be open for extension, but closed for modification."
3) Liskov Substitution Principle (LSP): "Objects in a program should be replaceable with instances of their subtypes without altering the correctness of that program."
4) Interface Segregation Principle (ISP): "Many client-specific interfaces are better than one general-purpose interface."
5) Dependency Inversion Principle (DIP): One should "depend upon abstractions, [not] concretions."

* DRY (Don’t Repeat Yourself)
1) "Every piece of knowledge must have a single, unambiguous, authoritative representation within a system

* KIS principle
1) Keep it simple 

* Law of Demeter (LoD)
1) a given object should assume as little as possible about the structure or properties of anything else (including its subcomponents), in accordance with the principle of "information hiding"

* Design by contract (DbC)
1) software designers should define formal, precise and verifiable interface specifications for software components, which extend the ordinary definition of abstract data types with preconditions, postconditions and invariants

* Encapsulation
1) bundling of data with the methods that operate on that data, or the restricting of direct access to some of an object's components.

* Command-Query-Separation(CQS)
1) "Functions should not produce abstract side effects...only commands (procedures) will be permitted to produce side effects."

* Principle of least astonishment (POLA)
1) a component of a system should behave in a way that most users will expect it to behave. The behavior should not astonish or surprise users

* Linguistic-Modular-Units
1) Modules must correspond to syntactic units in the language used

* Self-Documentation
1) The designer of a module should strive to make all information about the module part of the module itself.

* Uniform-Access
1) All services offered by a module should be available through a uniform notation, which does not betray whether they are implemented through storage or through computation.

* Single-Choice
1) Whenever a software system must support a set of alternatives, one and only one module in the system should know their exhaustive list.

* Persistence-Closure
1) Whenever a storage mechanism stores an object, it must store with it the dependents of that object. Whenever a retrieval mechanism retrieves a previously stored object, it must also retrieve any dependent of that object that has not yet been retrieved.
