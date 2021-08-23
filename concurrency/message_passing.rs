#![allow(warnings)]

// A channel in programming has two halves: a transmitter and a receiver. 
// The transmitter half is the upstream location where you put rubber ducks into the river, and 
// the receiver half is where the rubber duck ends up downstream. 

// we’ll work up to a program that has one thread to generate values and send them down a channel, and 
// another thread that will receive the values and print them out.

// Once you’re familiar with the technique, you could use channels to implement a system where many threads perform parts of a calculation and send the parts to one thread that aggregates the results.

use std::sync::mpsc; //  mpsc stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    // We create a new channel using the mpsc::channel function; Creating a channel and assigning the two halves to tx and rx
    // Using a let statement this way is a convenient approach to extract the pieces of the tuple returned by mpsc::channel
    let (tx, rx) = mpsc::channel(); // the tx element is the sending end and the rx element is the receiving end

    // Moving tx to a spawned thread and sending “hi”
    thread::spawn(move || { // Moving tx to a spawned thread and sending “hi”
        let val = String::from("hi");
        tx.send(val).unwrap(); // The transmitting end has a send method that takes the value we want to send.
        // The send method returns a Result<T, E> type, so if the receiving end has already been dropped and there’s nowhere to send a value, the send operation will return an error.
        // In this example, we’re calling unwrap to panic in case of an error. 
    });

    // Receiving the value “hi” in the main thread and printing it
    // The receiving end of a channel has two useful methods: recv and try_recv.

    let received = rx.recv().unwrap(); // recv will block the main thread’s execution and wait until a value is sent down the channel. Once a value is sent, recv will return it in a Result<T, E>.
    println!("Got: {}", received);
    // When the sending end of the channel closes, recv will return an error to signal that no more values will be coming.
    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. 

    // Using try_recv is useful if this thread has other work to do while waiting for messages: 
    // we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.


    //////// EXAMPLE 2 ////////
    // Sending Multiple Values and Seeing the Receiver Waiting
    // prove the code in is running concurrently
    // the spawned thread will now send multiple messages and pause for a second between each message.

    let (producerx, consumerx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            producerx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // In the main thread, we’re not calling the recv function explicitly anymore: 
    // instead, we’re treating consumerx as an iterator. For each value received, we’re printing it. When the channel is closed, iteration will end.
    for received2 in consumerx { // Because we don’t have any code that pauses or delays in the for loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.
        println!("Got: {}", received2);
    }



    //////// EXAMPLE 3 ////////
    // Creating Multiple Producers by Cloning the Transmitter
    // Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer. Let’s put mpsc
    let (ttx, rrx) = mpsc::channel();

    let tx1 = ttx.clone(); // This time, before we create the first spawned thread, we call clone on the sending end of the channel. 
    // This will give us a new sending handle we can pass to the first spawned thread
    thread::spawn(move || { // sending handle we can pass to the first spawned thread
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || { // We pass the original sending end of the channel to a second spawned thread.
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            ttx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // The above gives us two threads, each sending different messages to the receiving end of the channel.

    for received3 in rrx {
        println!("Got: {}", received3);
    }

}




