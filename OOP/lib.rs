#![allow(warnings)]


// Characteristics of Object-Oriented Languages
// Objects Contain Data and Behavior
// Encapsulation that Hides Implementation Details

pub struct AveragedCollection { // An AveragedCollection struct that maintains a list of integers and the average of the items in the collection
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection { // Implementations of the public methods add, remove, and average on AveragedCollection
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Inheritance as a Type System and as Code Sharing
// Inheritance is a mechanism whereby an object can inherit from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.
// There is no way to define a struct that inherits the parent struct’s fields and method implementations. 

// You choose inheritance for two main reasons.

// One is for reuse of code: you can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type.
// You can share Rust code using default trait method implementations 

// The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type. 
// This is also called polymorphism, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

// For these reasons, Rust takes a different approach, using trait objects instead of inheritance. Let’s look at how trait objects enable polymorphism in Rust.