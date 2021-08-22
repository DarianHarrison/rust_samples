#![allow(warnings)]
#[derive(Debug)] 

// a match allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 

enum Coin { // A Coin enum in which the Quarter variant also holds a UsState value
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), 
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // list the match keyword
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {

	// Combining match and enums is useful in many situations. 
	// Match against an enum, bind a variable to the data inside, and then execute code based on it.
	//It's consistently a user favorite.

	// A function that uses a match expression on an Option<i32>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{} {} {}", five,six,none);
}
