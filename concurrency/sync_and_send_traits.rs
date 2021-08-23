#![allow(warnings)]

// Allowing Transference of Ownership Between Threads with Send
// The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads. 
// Almost every Rust type is Send

// Allowing Access from Multiple Threads with Sync
// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
// In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread. 


// Implementing Send and Sync Manually Is Unsafe

fn main() {}