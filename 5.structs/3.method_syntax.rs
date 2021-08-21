// Methods are similar to functions, they are declared with the fn
// However, methods are different from functions in that they’re defined within the context of a struct.
// A Methods first parameter is always self, which represents the instance of the struct the method is being called on.

#![allow(dead_code)] // disable warning
#[derive(Debug)] // used for logging structs to std output

struct Rectangle {
    width: u32,
    height: u32,
}

// Each struct is allowed to have multiple impl blocks.

// Defining an area method on the Rectangle struct
impl Rectangle { // To define the function within the context of Rectangle, we start an impl (implementation) block.
    // we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle due to this method’s being inside the impl Rectangle context.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Implementing the can_hold method on Rectangle that takes another Rectangle instance as a parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// Associated Functions
// impl blocks that don’t take self as a parameter. 
// Associated functions are often used for constructors that will return a new instance of the struct. 
impl Rectangle { // To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


// In the signature for area, we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle due to this method’s being inside the impl Rectangle context.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // In Rust when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method.

    println!("Can rect1 hold rect2? {}", (&rect1).can_hold(&rect2)); 
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // same as above, this is cleaner    
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);


}

