# Chapter 1

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
cargo run # to compile and run
cargo check # run cargo check periodically to make sure it compiles, (does not compile code)
```
Production Build
```
cargo build --release # compile it with optimizations
```

# Chapter 2

```
cargo new guessing_game
cd guessing_game
```
```
cat << 'EOF' > Cargo.toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.3"
EOF
```
```
cat << 'EOF' > src/main.rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // gen_range method takes a range expression as an argument and generates a random number in the range.
    let secret_number = rand::thread_rng().gen_range(1..101); //The kind of range expression we’re using here takes the form start..end 

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // a mutable variable that is currently bound to a new, empty instance of a String

        io::stdin()
            .read_line(&mut guess) // & is a reference to a variable to prevent copying data all over the place in the code
            .expect("Failed to read line"); // If this instance of io::Result is an Err value, expect will cause the program to crash and display the message.


	    // Rust allows us to shadow the previous value of guess with a new one, This feature is often used in situations in which you want to convert a value from one type to another type.

	    // a match expression is how you generally handling an error.
        let guess: u32 = match guess.trim().parse() { // parse returns a Result type and Result is an enum that has the variants Ok or Err
            Ok(num) => num, // the match expression will just return the num value that parse produced and put inside the Ok value
            Err(_) => continue, //The underscore, _, is a catchall value; match all Err values, no matter what information they have inside them. 
        };

        println!("You guessed: {}", guess);

        // A match expression is made up of arms. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
EOF
```
```
cargo check # run cargo check periodically to make sure it compiles, (does not compile code)
cargo update # update pachages in Cargo.toml
cargo run # to compile and run
```

# Chapter 3



```
cargo new programming_concepts
cd programming_concepts
```
```
cat << 'EOF' > Cargo.toml
[package]
name = "programming_concepts"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
```
cat << 'EOF' > src/main.rs
fn main() {


	//////////// VARIABLES ////////////


	// Mutable var
    let mut x = 5;
    x = 6;


    // Immutable var
    let y = 5;


    // Constants, 
    // Constants type of the value must be annotated
    // Constants can be declared in any scope, including the global scope, and are valid for the entire time a program runs, within the scope they were declared in.
    // constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;


    // Shadowing, second variable’s value is what appears when the variable is used
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    let z = 5; // 5
    let z = z + 1; // 6
    let z = z * 2; // 12


    //////////// DATA TYPES ////////////


    // Integer Types
    // number without a fractional component.
    // Default will is i32
    // Unsigned variants can store numbers from 0 to 2^n - 1, so a "u8" can store numbers from 0 to 2^8 - 1, which equals 0 to 255.
    // Signed variant "i8" can store numbers from -128 to 127
    // The "isize" and "usize" types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture. // almost never used.


    // Floating-Point Types
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    // for best results 
    let xx = 2.0; // f64
    let yy: f32 = 3.0; // f32


    //Numeric Operations
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder


    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation


    // Tuple Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup; // we can use pattern matching to destructure a tuple value; y = 6.4
	let five_hundred = x.0; // we can access a tuple element directly 
	

	// Array Type
	// arrays in Rust have a fixed length, like tuples.
	// Unlike a tuple, every element of an array must have the same type
	// Arrays are useful when you want your data allocated on the stack rather than the heap
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	let first = a[0]; // Accessing Array Elements


	// Vector Type
	//A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
	let v: Vec<i32> = Vec::new(); // we’ve told Rust that the Vec<T> in v will hold elements of the i32 type.


    //////////// FUNCTIONS ////////////

    // the main function is the entry point of many programs.
	fn another_function() {
	    println!("Another function.");
	}


}

}
EOF
```

```
cargo check # run cargo check periodically to make sure it compiles, (does not compile code)
cargo update # update pachages in Cargo.toml
cargo run # to compile and run
```

# Chapter 4
# Chapter 5
# Chapter 6
# Chapter 7
# Chapter 8
# Chapter 9
