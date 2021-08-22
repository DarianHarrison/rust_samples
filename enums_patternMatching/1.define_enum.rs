#![allow(warnings)]

fn main() {

	// We can represent the same concept by putting data directly into each enum variant.
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

	// Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Example 2: A Message enum whose variants each store different amounts and types of values
    // Defining an enum is similar to defining different kinds of struct definitions, except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
	enum Message {
	    Quit, // unit struct
	    Move { x: i32, y: i32 },
	    Write(String), // tuple struct
	    ChangeColor(i32, i32, i32), // tuple struct
	}

	// we’re also able to define methods on enums
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();


    // Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>
	// enum Option<T> { // The <T> syntax is a generic type parameter
	//     Some(T),
	//     None,
	// }

	// use Some and None directly without the Option:: prefix. 
	// The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
    let some_number = Some(5);
    let some_string = Some("a string");
    // If we use None rather than Some, we need to tell Rust what type of Option<T> we have,
    // because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.
    let absent_number: Option<i32> = None;
}