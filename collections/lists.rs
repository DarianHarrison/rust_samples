#![allow(warnings)]


fn main() {
	// create a vector
	let v: Vec<i32> = Vec::new();

	// Updating a Vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // Reading Elements of Vectors
    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }    

    // Iterating over the Values in a Vector

    // immutable
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    // mutable
    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
        println!("{}", i);
    }

    // Defining an enum to store values of different types in one vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


} // <- vn goes out of scope and is freed here