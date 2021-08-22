#![allow(warnings)]
use std::fmt::{Display, Debug};

// A trait tells the Rust compiler about functionality a particular type has and can share with other types.
// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

// Implementing a Trait on a Type
// Implementing the Summary trait on the NewsArticle and Tweet types


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String { // default behavour
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// Traits as Parameters
//  use traits to define functions that accept many different types.


pub fn notify<T: Summary>(item1: &T, item2: &T) {// Trait Bounds
    println!("Breaking news!");
}

// Clearer Trait Bounds with where Clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    let a = 123;
    a
}

// Fixing the largest Function with Trait Bounds
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


    // Default Implementations
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


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
