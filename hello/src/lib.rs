pub struct ThreadPool;

impl ThreadPool {
    // create an associated function named new for ThreadPool 
    pub fn new(size: usize) -> ThreadPool { // We chose usize as the type of the size parameter, because we know that a negative number of threads doesn’t make any sense.  We also know we’ll use this 4 as the number of elements in a collection of threads, which is what the usize ype is for
        ThreadPool
    }


    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

}



// 
