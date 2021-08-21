#![allow(warnings)]

// A User struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    // Creating an immutable instance of the User struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // create a mutable instance of the User struct
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Changing the value in the email field of a User instance
    user2.email = String::from("anotheremail@example.com");

    // Using struct update syntax to set new email and username values for a User instance but use the rest of the values from the fields of the instance in the user1 variable
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2
    };

    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


// A build_user function that takes an email and username and returns a User instance
// It uses field init shorthand because the email and username parameters have the same name as struct fields
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

