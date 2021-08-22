#![allow(warnings)]

// Drop lets you customize what happens when a value is about to go out of scope. 
// the Drop trait is almost always used when implementing a smart pointer. For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.



//  A CustomSmartPointer struct
struct CustomSmartPointer {
    data: String,
}

// A CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    
	//  Calling std::mem::drop to explicitly drop a value before it goes out of scope
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");


    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");



}


