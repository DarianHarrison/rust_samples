#![allow(warnings)]

use std::fmt::Display;

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. 
// Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}



fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+


    {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };



    // The Static Lifetime
    let s: &'static str = "I have a static lifetime.";


    // full example
    let string3 = String::from("full example");
    let string4 = "xyz";

    let result2 = longest_with_an_announcement(
        string3.as_str(),
        string4,
        "Today is someone's birthday!",
    );
    println!("The longest string is: {}", result2);

}


/*Lifetime annotations have a slightly unusual syntax: 
the names of lifetime parameters must start with an apostrophe ('), and 
are usually all lowercase and very short, like generic types. 
Most people use the name 'a. We place lifetime parameter annotations after the & of a reference, 
using a space to separate the annotation from the reference’s type.*/

// The longest function definition specifying that all the references in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
