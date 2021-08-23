use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // The bind function in this scenario works like the new function in that it will return a new TcpListener instance.

    for stream in listener.incoming() { // Listening for incoming streams and printing a message when we receive a stream
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


// In the handle_connection function, we’ve made the stream parameter mutable. 
// The reason is that the TcpStream instance keeps track of what data it returns to us internally. 
// It might read more data than we asked for and save that data for the next time we ask for data. 
// It therefore needs to be mut because its internal state might change; 
// usually, we think of “reading” as not needing mutation, but in this case we need the mut keyword.
fn handle_connection(mut stream: TcpStream) { 

    // Next, we need to actually read from the stream. 
    // We do this in two steps: 

    // first, we declare a buffer on the stack to hold the data that is read in. 
    let mut buffer = [0; 1024]; // We’ve made the buffer 1024 bytes in size, which is big enough to hold the data of a basic request. 
    // If we wanted to handle requests of an arbitrary size, buffer management would need to be more complicated; we’ll keep it simple for now. 

    // We pass the buffer to stream.read, which will read bytes from the TcpStream and put them in the buffer.
    stream.read(&mut buffer).unwrap();


    //  Matching the request and handling requests to / differently from other requests
    let get = b"GET / HTTP/1.1\r\n"; // First, we hardcode the data corresponding to the / request into the get
    // Because we’re reading raw bytes into the buffer, we transform get into a byte string by adding the b"" byte string syntax at the start of the content data. 


    // we check whether buffer starts with the bytes in get.
    let (status_line, filename) = if buffer.starts_with(get) { // If it does, it means we’ve received a well-formed request to /, which is the success case we’ll handle in the if block that returns the contents of our HTML file.
        ("HTTP/1.1 200 OK", "hello.html")
    } else { // If buffer does not start with the bytes in get, it means we’ve received some other request. We’ll add code to the else block in a moment to respond to all other requests.
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    // Writing a tiny successful HTTP response to the stream
    let response = format!( // we use format! to add the file’s contents as the body of the success response. 
        "{}\r\nContent-Length: {}\r\n\r\n{}", // To ensure a valid HTTP response, we add the Content-Length header which is set to the size of our response body, in this case the size of hello.html.
        status_line,
        contents.len(),
        contents // Sending the contents of hello.html as the body of the respons
    );

    // Then we call as_bytes on our response to convert the string data to bytes.
    // The write method on stream takes a &[u8] and sends those bytes directly down the connection.
    stream.write(response.as_bytes()).unwrap( /* in a real application you would add error handling here.*/ ); // Because the write operation could fail, we use unwrap on any error result as before.
    stream.flush().unwrap();  // Finally, flush will wait and prevent the program from continuing until all the bytes are written to the connection;

}





