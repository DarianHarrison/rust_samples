#![allow(warnings)]
fn main() {

    // 
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


    // Mutable References
    let mut s2 = String::from("hello"); // First, change s2 to be mut
    change(&mut s2); // Then create a mutable reference with &mut s2 and accept a mutable reference with some_string: &mut String.


    // prevent data race (allow for mutation but in a very controlled fashion.)
    // we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    let mut s4 = String::from("hello");

    {
        let r1 = &mut s4;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let s5 = &mut s4;



    let mut s6 = String::from("hello");

    let r1 = &s6; // no problem
    let r2 = &s6; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s6; // no problem
    println!("{}", r3);

    let reference_to_nothing = no_dangle();



    // The Slice Type, Another data type that does not have ownership

    //string slice
    let sss = String::from("hello world");

    let hello = &sss[0..5]; // .. range syntax
    let world = &sss[6..11];
    let slice = &sss[..2];
    let slice = &sss[3..];


    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

}

fn calculate_length(s: &String) -> usize { // define and use a function that has a reference to an object as a parameter instead of taking ownership of the value
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens


fn change(some_string: &mut String) { // accept a mutable reference with some_string: &mut String
    some_string.push_str(", world");
}


fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

