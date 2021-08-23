#![allow(warnings)]

use std::thread;
use std::time::Duration;

fn main() {
	// Waiting for All Threads to Finish Using join Handles
	// Notice in that the closure we pass to thread::spawn takes no arguments: we're not using any data from the main thread in the spawned thread’s code.
    let handle = thread::spawn(|| {// To create a new thread, we call the thread::spawn function and pass it a closure
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. 
    // Blocking a thread means that thread is prevented from performing work or exiting. 
    // Because we’ve put the call to join after the main thread’s for loop, running Listing 16-2 should produce output similar to this:
    handle.join().unwrap();


    // Using move Closures with Threads
    // The move closure is often used alongside thread::spawn because it allows you to use data from one thread in another thread.

    // To use data from the main thread in the spawned thread, 
    // the spawned thread’s closure must capture the values it needs. 
    // create a vector in the main thread and use it in the spawned thread. 

    let v = vec![1, 2, 3];
    
    let handle2 = thread::spawn(move || { // Using the move keyword to force a closure to take ownership of the values it uses
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();

}
