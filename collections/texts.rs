#![allow(warnings)]


fn main() {

    // Creating a New String
    let mut s = String::new();

    let data = "initial contents";
    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s5 is {}", s5);

    // Concatenation
    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    let s9 = format!("{}-{}-{}", s6, s7, s8);

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    
} 