
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

```


```

# 8. Common Collections

```


```

# 9. Error Handling

```


```

# 10. Generic Types, Traits, and Lifetimes

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