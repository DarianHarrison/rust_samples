use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


// Concurrency actors (spawning threads):
// 1. Define a Worker struct that holds an id and a JoinHandle<()>.
// 2. Change ThreadPool to hold a vector of Worker instances.
// 3. Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
// 4. In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.

// Concurrency channels (closures):
//We’ll use a channel to function as the queue of jobs, and execute will send a job from the ThreadPool to the Worker instances, which will send the job to its thread. Here is the plan:
// 1. The ThreadPool will create a channel and hold on to the sending side of the channel.
// 2. Each Worker will hold on to the receiving side of the channel.
// 3. We'll create a new Job struct that will hold the closures we want to send down the channel.
// 4. The execute method will send the job it wants to execute down the sending side of the channel.
// 5. In its thread, the Worker will loop over its receiving side of the channel and execute the closures of any jobs it receives.

pub struct ThreadPool { // 1. The ThreadPool will create a channel and hold on to the sending side of the channel.
    workers: Vec<Worker>, // // 2. Each Worker will hold on to the receiving side of the channel.
    sender: mpsc::Sender<Message>,
}


// We’ll also change Job from a struct to a type alias for a trait object that holds the type of closure that execute receives.
// A Job type alias
type Job = Box<dyn FnOnce() + Send + 'static>; // 3. We'll create a new Job struct that will hold the closures we want to send down the channel.


pub struct ThreadPool { // 2. Change ThreadPool to hold a vector of Worker instances.
    workers: Vec<Worker>,
}

impl ThreadPool {

    // create an associated function named new for ThreadPool 
    pub fn new(size: usize) -> ThreadPool { // We chose usize as the type of the size parameter, because we know that a negative number of threads doesn’t make any sense.  We also know we’ll use this 4 as the number of elements in a collection of threads, which is what the usize ype is for

        assert!(size > 0); // Implementing ThreadPool::new to panic if size is zero

        let (sender, receiver) = mpsc::channel(); // let mut workers = Vec::with_capacity(size);

        // The Arc type will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time. 
        let receiver = Arc::new(Mutex::new(receiver));


        // declare the threads vector
        let mut workers = Vec::with_capacity(size);

        // create some threads and store them in the vector
        for _ in 0..size { // 4. In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.
            workers.push(Worker::new(id)); 
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) // We’ll define the execute method on ThreadPool to take a closure as a parameter. 
    where

        // spawn uses FnOnce as the trait bound on F. This is probably what we want here as well, because we’ll eventually pass the argument we get in execute to spawn.
        // We can be further confident that FnOnce is the trait we want to use because the thread for running a request will only execute that request’s closure one time, which matches the Once in FnOnce.
        // We use the () after FnOnce because this FnOnce represents a closure that takes no parameters and returns the unit type ()
        // The F type parameter also has the trait bound Send and the lifetime bound 'static, which are useful in our situation: we need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute.
        F: FnOnce() + Send + 'static, 
        
    { // let's implement the execute method on ThreadPool.

        let job = Box::new(f); // a Box that holds each closure
        self.sender.send(Message::NewJob(job)).unwrap(); // and then sending the job down the channel

    }

}

// A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread
struct Worker { // 1. Define a Worker struct that holds an id and a JoinHandle<()>.
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // a Worker that is running will have a Some variant in thread, and when we want to clean up a Worker, we’ll replace Some with None so the Worker doesn’t have a thread to run
}

// 3. Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
impl Worker { // 
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker { // Receiving and executing the jobs in the worker’s thread
        let thread = thread::spawn(move || loop { // our closure being passed to thread::spawn  references the receiving end of the channel
            let message = receiver.lock().unwrap().recv().unwrap(); // Here, we first call lock on the receiver to acquire the mutex, and then we call unwrap to panic on any errors.

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread), // we need to wrap the thread value in Some when we create a new Worker
        }
    }
}



// Implementing the Drop Trait on ThreadPool
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        
        for worker in &mut self.workers { // Joining each thread when the thread pool goes out of scope
            println!("Shutting down worker {}", worker.id);

            // We’re using if let to destructure the Some and get the thread; then we call join on the thread. 
            // If a worker’s thread is already None, we know that worker has already had its thread cleaned up, so nothing happens in that case.
            if let Some(thread) = worker.thread.take() { // the take method on Option takes the Some variant out and leaves None in its place. 
                thread.join().unwrap(); 
            }
        }
    }
}


// This Message enum will either be a NewJob variant that holds the Job the thread should run, or it will be a Terminate variant that will cause the thread to exit its loop and stop.
enum Message {
    NewJob(Job),
    Terminate,
}