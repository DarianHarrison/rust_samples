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
EOF
```
```
cat << 'EOF' > src/main.rs
fn main() { // the main function is the entry point of many programs.


	//////////// VARIABLES ////////////


	// Mutable var
    let mut x = 5;
    x = 6;
    println!("{}",x)


    // Immutable var
    let y = 5;
    println!("{}",y)


    // Constants, 
    // Constants type of the value must be annotated
    // Constants can be declared in any scope, including the global scope, and are valid for the entire time a program runs, within the scope they were declared in.
    // constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;
    println!("{}",MAX_POINTS)


    // Shadowing, second variable’s value is what appears when the variable is used
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    let z = 5; // 5
    let z = z + 1; // 6
    let z = z * 2; // 12
    println!("{}",z)


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
    println!("{},{}",xx, yy)


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
	let tuple_x: (i32, f64, u8) = (500, 6.4, 1);
	let (xxx, yyy, zzz) = tuple_x; // we can use pattern matching to destructure a tuple value; yyy = 6.4
	let five_hundred = tuple_x.0; // we can access a tuple element directly 
	

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


    // functions and expressions
	fn another_function(x: i32) -> i32 { // declare a function that takes in an signed int type and returns signed int type
		let y: i32 = 6;
	    let z: i32 = { // Expressions evaluate to something
	        x + y
	    };

        return z
	}

	let zz: i32 = another_function(5); // call the function
    println!("{}", zz);


    //////////// CONTROL FLOW ////////////


    //if Expressions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }  


    //if in a let Statement
    let condition = true;
    let numberx = if condition { 5 } else { 6 }; // ternary
    println!("The value of numberx is: {}", numberx);    


    // while loops
    let some_array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", some_array[index]);
        index += 1;
    }


    // for loops
    for e in (1..4).rev() {
        println!("{}!", e);
    }
    println!("LIFTOFF!!!");


}
EOF
```

```
cargo check
cargo update
cargo run
```

# Chapter 4

```
cargo new ownership
cd ownership
```
```
cat << 'EOF' > Cargo.toml
[package]
name = "ownership"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
EOF
```
```
cat << 'EOF' > src/main.rs

fn main() {

    //////////// STACK AND  HEAP ////////////
    // The stack stores values in the order it gets them and removes the values in the opposite order.
    // All data stored on the stack must have a known, fixed size. 
    // Data with an unknown size at compile time or a size that might change must be stored on the heap instead. 
    // Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack
    // Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 
    // When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. 
    // When the function is over, those values get popped off the stack.
    // Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
    // managing heap data is why ownership exists


    //////////// OWNERSHIP RULES ////////////
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // To illustrate the rules of ownership, we need a data type that is more complex than the ones we covered in the “Data Types”
    // The types covered previously are all stored on the stack and popped off the stack when their scope is over, but we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data.
    // We’ll use String as the example here and concentrate on the parts of String that relate to ownership.
    // These aspects also apply to other complex data types, whether they are provided by the standard library or created by you.

    // Rust has a second string type, String. This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    // You can create a String from a string literal using the from function
    let mut s = String::from("hello"); // (::) is an operator that allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`


    // when s2 and s1 go out of scope, they will both try to free the same memory. 
    // This is known as a double free error and is one of the memory safety bugs we mentioned previously.
    // let s1 = String::from("hello");
    // let s2 = s1; // Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    // You’ll get an error like this because Rust prevents you from using the invalidated reference

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    // heap data does get copied.
    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
    let s5 = String::from("hello");
    let s6 = s1.clone();
    println!("s5 = {}, s6 = {}", s5, s6);

    // Stack-Only Data: Copy
    // the bellow code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. 
    // That means there’s no reason we would want to prevent x from being valid after we create the variable y. 
    // In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out.
    let xx = 5;
    let yy = xx;
    println!("xx = {}, yy = {}", xx, yy);
    // Here are some of the types that implement Copy:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


}


//////////// MEMORY AND ALLOCATION ////////////

fn calculate_length(s3: String) -> (String, usize) { 


    //////////// VARIABLE SCOPE ////////////
    // Rust returns memory once the variable that owns it goes out of scope.
    let s4 = "hello"; // s4 is valid from this point forward

    // do stuff with s4

    let length = s3.len(); // len() returns the length of a String
    (s3, length)

} // this scope is now over, and s4 IS no longer valid
// Rust calls a special function called drop, where the String s4 can return the memory. 
// Rust calls drop automatically at the closing curly bracket.

EOF
```
```
cargo check
cargo update
cargo run
```


# Chapter 5
# Chapter 6
# Chapter 7
# Chapter 8
# Chapter 9
