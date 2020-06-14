// Web Server in pure Rust
// Go to 127.0.0.1:7878 in your browser. 
// Single-threaded web server: process each request in turn, won't process a 2nd request connection until the first is done.

// Using the net module to listen to a TCP connection: 
use std::net::TcpListener;

// Using std::io::prelude to get access to certain traits that let us read from and write to the stream: 
use std::io::prelude::*;
use std::net::TcpStream;

// Using the std lib filesystem module to read files: 
use std::fs;

fn main() {
    // # Listening to the TCP connection: 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();        // "local-IP-address: port".
    // bind fn returns a new instance of TcpListener - connecting to a port to listen to, aka binding to a port. 

    for stream in listener.incoming() {
        // streams of type TcpStream: 
        let stream = stream.unwrap();   // a stream represents an open connection between client & server. 
        println!("Connection established!");

        handle_connection(stream);
    }
}

// # Reading the request from the browser and writing a response! Using the fn "handle_connection" for processing connections.
pub fn handle_connection(mut stream: TcpStream) {
    // TcpStream keeps an internal track of what data it returns. 
    let mut buffer = [0; 512];      // buffer on the stack to hold the data that is read in (512 bytes in size). 

    stream.read(&mut buffer).unwrap();      // .read bytes from stream and put them in the buffer. 

    // # Printing the request data:
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));      // &[u8] as input. 

    // # Responses have the following format:
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    let success_response = "HTTP/1.1 200 OK\r\n\r\n";       // standard success response - no headers and no body. 

    // The status code 404 signals that the content for the request was not found: 
    let status_line_404 = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    // # Functionality to check what the browser is requesting:
    let get = b"GET / HTTP/1.1\r\n";        // hardcode the data corresponding to the / requests into get variable. 
                                            // b"" byte string syntax to transform get into raw bytes. 

    let (status_line, filename) = if buffer.starts_with(get) {
        (success_response, "hello.html")
    } else {
        (status_line_404, "404.html")
    };

    // Return the HTML:
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();     // .write takes a &[u8] and sends those bytes down the connection. 
    stream.flush().unwrap();
    // .flush waits & prevents the program from continuing until all bytes are written to the connection. 
}