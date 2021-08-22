#![allow(warnings)]
// Let’s first look at how to define functions, structs, enums, and methods using generics

// In Function Definitions
// Two functions that differ only in their names and the types in their signatures
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// In Enum Definitions
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// In Struct Definitions
// generic type parameter
struct Point<T> {
    x: T,
    y: T,
}


// A Point2<T, U> generic over two types so that x and y can be values of different types
struct Point2<T, U> {
    x: T,
    y: U,
}

// In Method Definitions
struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// A method that uses different generic types from its struct’s definition
struct Point4<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point4<T, U> {
    fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
        Point4 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');



    // Point
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };


    // Point2
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };



    // method definition call
    let p = Point3 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    //  A method that uses different generic types from its struct’s definition
    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
