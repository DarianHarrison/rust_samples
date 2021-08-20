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
cargo update
cargo run # to compile and run
cargo check # run cargo check periodically to make sure it compiles, (does not compile code)
```






# Chapter 3
# Chapter 4
# Chapter 5
# Chapter 6
# Chapter 7
# Chapter 8
# Chapter 9
