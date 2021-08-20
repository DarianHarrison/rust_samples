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
