#![allow(warnings)]


// Using Mutexes to Allow Access to Data from One Thread at a Time
// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
// similar to a panel and a microphone, where only one pannelist may speack at a time.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() { 

    // Exploring the API of Mutex<T> in a single-threaded context for simplicity
    let m = Mutex::new(5); // we create a Mutex<T> using the associated function new.

        // Mutex<T> is a smart pointer. 
        // More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap. 
        // The MutexGuard smart pointer implements Deref to point at our inner data; 
        // the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope, which happens at the end of the inner scope listed bellow. 
        // As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads because the lock release happens automatically.
        //After dropping the lock, we can print the mutex value and see that we were able to change the inner 

    {
        let mut num = m.lock().unwrap(); // To access the data inside the mutex, we use the lock method to acquire the lock. 
        // This call to lock will block the current thread so it can’t do any work until it’s our turn to have the lock.
        // The call to lock would fail if another thread holding the lock panicked. In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation.

        // After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside. 
        // The type system ensures that we acquire a lock before using the value in m: Mutex<i32> is not an i32, so we must acquire the lock to be able to use the i32 value. 
        // We can’t forget; the type system won’t let us access the inner i32 otherwise.
        *num = 6;
    }
    println!("m = {:?}", m);



    /////////////// EXAMPLE 2 ///////////////
    // Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads

    // Atomic Reference Counting with Arc<T> 
    // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
    // The a stands for atomic, meaning it’s an atomically reference counted type.
    // We’ll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10. The next example in Listing
    // The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to. 
    // If you’re just performing operations on values within a single thread, your code can run faster if it doesn’t have to enforce the guarantees atomics provide.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());


    // Using this strategy, you can divide a calculation into independent parts, split those parts across threads, and then use a Mutex<T> to have each thread update the final result with its part.
}

// Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.