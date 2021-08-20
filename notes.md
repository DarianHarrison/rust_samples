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
use std::io;
use rand::Rng; // The Rng trait defines methods that random number generators implement

fn main() {
    println!("Guess the number!");

    // gen_range method takes a range expression as an argument and generates a random number in the range.
    let secret_number = rand::thread_rng().gen_range(1..101); //The kind of range expression we’re using here takes the form start..end 

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // a mutable variable that is currently bound to a new, empty instance of a String

    io::stdin()
        .read_line(&mut guess) // & is a reference to a variable to prevent copying data all over the place in the code
        .expect("Failed to read line"); // If this instance of io::Result is an Err value, expect will cause the program to crash and display the message.

    println!("You guessed: {}", guess);
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
