#![allow(warnings)]

fn main() {

    //////////// STACK AND  HEAP ////////////
    // The stack stores values in the order it gets them and removes the values in the opposite order.
    // All data stored on the stack must have a known, fixed size. 
    // Data with an unknown size at compile time or a size that might change must be stored on the heap instead. 
    // Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack
    // Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 
    // When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. 
    // When the function is over, those values get popped off the stack.
    // Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
    // managing heap data is why ownership exists


    //////////// OWNERSHIP RULES ////////////
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // To illustrate the rules of ownership, we need a data type that is more complex than the ones we covered in the “Data Types”
    // The types covered previously are all stored on the stack and popped off the stack when their scope is over, but we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data.
    // We’ll use String as the example here and concentrate on the parts of String that relate to ownership.
    // These aspects also apply to other complex data types, whether they are provided by the standard library or created by you.

    // Rust has a second string type, String. This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    // You can create a String from a string literal using the from function
    let mut s = String::from("hello"); // (::) is an operator that allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`


    // when s2 and s1 go out of scope, they will both try to free the same memory. 
    // This is known as a double free error and is one of the memory safety bugs we mentioned previously.
    // let s1 = String::from("hello");
    // let s2 = s1; // Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    // You’ll get an error like this because Rust prevents you from using the invalidated reference

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    // heap data does get copied.
    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
    let s5 = String::from("hello");
    let s6 = s5.clone();
    println!("s5 = {}, s6 = {}", s5, s6);

    // Stack-Only Data: Copy
    // the bellow code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. 
    // That means there’s no reason we would want to prevent x from being valid after we create the variable y. 
    // In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out.
    let xx = 5;
    let yy = xx;
    println!("xx = {}, yy = {}", xx, yy);
    // Here are some of the types that implement Copy:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.




    let s123 = String::from("hello");  // s123 comes into scope
    takes_ownership(s123);             // s123's value moves into the function...
                                       // ... and so is no longer valid here

    let zz = 5;                      // zz comes into scope

    makes_copy(zz);                  // zz would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use zz afterward


} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.






//////////// MEMORY AND ALLOCATION ////////////
fn calculate_length(s3: String) -> (String, usize) { 
    //////////// VARIABLE SCOPE ////////////
    // Rust returns memory once the variable that owns it goes out of scope.
    let s4 = "hello"; // s4 is valid from this point forward

    // do stuff with s4
    println!("{}",s4);

    let length = s3.len(); // len() returns the length of a String
    println!("{}",length);
    (s3, length)

} // this scope is now over, and s4 IS no longer valid
// Rust calls a special function called drop, where the String s4 can return the memory. 
// Rust calls drop automatically at the closing curly bracket.

