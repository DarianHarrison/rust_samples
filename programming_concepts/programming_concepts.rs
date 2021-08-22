#![allow(warnings)]

fn main() { // the main function is the entry point of many programs.


    //////////// VARIABLES ////////////


    // Mutable var
    let mut x = 5;
    x = 6;
    println!("{}",x);


    // Immutable var
    let y = 5;
    println!("{}",y);


    // Constants, 
    // Constants type of the value must be annotated
    // Constants can be declared in any scope, including the global scope, and are valid for the entire time a program runs, within the scope they were declared in.
    // constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    const MAX_POINTS: u32 = 100_000;
    println!("{}",MAX_POINTS);


    // Shadowing, second variable’s value is what appears when the variable is used
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    let z = 5; // 5
    let z = z + 1; // 6
    let z = z * 2; // 12
    println!("{}",z);


    //////////// DATA TYPES ////////////


    // Integer Types
    // number without a fractional component.
    // Default will is i32
    // Unsigned variants can store numbers from 0 to 2^n - 1, so a "u8" can store numbers from 0 to 2^8 - 1, which equals 0 to 255.
    // Signed variant "i8" can store numbers from -128 to 127
    // The "isize" and "usize" types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture. // almost never used.


    // Floating-Point Types
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    // for best results 
    let xx = 2.0; // f64
    let yy: f32 = 3.0; // f32
    println!("{},{}",xx, yy);


    //Numeric Operations
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder
    println!("{} {} {} {} {}",sum,difference,product,quotient,remainder);



    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{} {}",t,f);

    // Tuple Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
    let tuple_x: (i32, f64, u8) = (500, 6.4, 1);
    let (xxx, yyy, zzz) = tuple_x; // we can use pattern matching to destructure a tuple value; yyy = 6.4
    let five_hundred = tuple_x.0; // we can access a tuple element directly 
    println!("{} {}",xxx,five_hundred);
    

    // Array Type
    // arrays in Rust have a fixed length, like tuples.
    // Unlike a tuple, every element of an array must have the same type
    // Arrays are useful when you want your data allocated on the stack rather than the heap
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0]; // Accessing Array Elements
    println!("{}",first);


    // Vector Type
    //A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    let v: Vec<i32> = Vec::new(); // we’ve told Rust that the Vec<T> in v will hold elements of the i32 type.
    println!("{}",first);

    //////////// FUNCTIONS ////////////


    // functions and expressions
    fn another_function(x: i32) -> i32 { // declare a function that takes in an signed int type and returns signed int type
        let y: i32 = 6;
        let z: i32 = { // Expressions evaluate to something
            x + y
        };

        return z
    }

    let zz: i32 = another_function(5); // call the function
    println!("{}", zz);


    //////////// CONTROL FLOW ////////////


    //if Expressions
    let numbery = 6;
    if numbery % 4 == 0 {
        println!("number is divisible by 4");
    } else if numbery % 3 == 0 {
        println!("number is divisible by 3");
    } else if numbery % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }  


    //if in a let Statement
    let condition = true;
    let numberx = if condition { 5 } else { 6 }; // ternary
    println!("The value of numberx is: {}", numberx);    


    // while loops
    let some_array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", some_array[index]);
        index += 1;
    }


    // for loops
    for e in (1..4).rev() {
        println!("{}!", e);
    }
    println!("LIFTOFF!!!");
}