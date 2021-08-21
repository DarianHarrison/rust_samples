
//Structs are similar to tuples


#![allow(dead_code)] // disable warning

fn main() {

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // To use an immutable struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // create a mutable struct
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //To get a specific value from a struct, we can use dot notation
    user2.email = String::from("anotheremail@example.com");

    // build_user function that returns a User instance with the given email and username.
    let user3 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );


    //Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    // you can also define structs that look similar to tuples, called tuple structs.
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

