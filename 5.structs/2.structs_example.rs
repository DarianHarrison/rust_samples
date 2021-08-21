#![allow(warnings)]
#[derive(Debug)] // Adding the annotation to derive the Debug trait

// Specifying the width and height of the rectangle with a Structs to Adding More Meaning
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

	// Tuple Case
    let rect1 = (30, 50); // Specifying the width and height of the rectangle with a tuple for more efficiency
    println!( "The area of the rectangle is {} square pixels.", area(rect1) );

    // Struct Case
    let rect2 = Rectangle { // add labels to data to add more meaning to the codes
        width: 30,
        height: 50,
    };
    println!("rect2 is {:?}", rect2); // printing the Rectangle instance using debug formatting
}

// Tuple Case
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Struct Case
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
