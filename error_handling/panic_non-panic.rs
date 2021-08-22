#![allow(warnings)]

// Recoverable Errors with Result
use std::fs::File;
use std::io::ErrorKind;

fn main() {

	// Matching on Different Errors
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });



    // A Shortcut for Propagating Errors: the ? Operator

	// the ? at the end of the File::open call will return the value inside an Ok to the variable f. 
	// If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. 
	// The same thing applies to the ? at the end of the read_to_string call.
	// Shortcuts for Panic on Error: unwrap and expect
	use std::io;
	use std::io::Read;

	fn read_username_from_file() -> Result<String, io::Error> {
	    let mut s = String::new();

	    File::open("hello.txt")?.read_to_string(&mut s)?;

	    Ok(s)
	}


	// this is specifig way of doing it for reading files, the above is done to showcase ? operator
	// Using fs::read_to_string instead of opening and then reading the file
	use std::fs;

	fn read_username_from_file2() -> Result<String, io::Error> {
	    fs::read_to_string("hello.txt")
	}

}
