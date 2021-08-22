#![allow(warnings)]

// A test for can_hold that checks whether a larger rectangle can indeed hold a smaller rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Testing Equality with the assert_eq! and assert_ne! Macros
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Adding Custom Failure Messages
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}


// Checking for Panics with should_panic with a particular panic message
pub struct Guess {
    value: i32,
}

// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// Testing a private function
pub fn add_four(a: i32) -> i32 {
    internal_adder(a, 4)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    // Checking Results with the assert! Macro
    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // Checking Results with the assert! Macro
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Testing Equality with the assert_eq! and assert_ne! Macros
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Custom Failure Messages
    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // Testing that a condition will cause a panic! with a particular panic message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn unit_test_sample() {
        assert_eq!(2 + 2, 4);
    }


    // Using Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Testing a private function
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

}