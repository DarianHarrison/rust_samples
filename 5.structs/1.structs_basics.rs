
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


}

