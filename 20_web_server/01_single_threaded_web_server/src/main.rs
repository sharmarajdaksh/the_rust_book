use std::io::prelude::*;
// We bring std::io::prelude into scope to get access to certain traits that
// let us read from and write to the stream

use std::net::{TcpListener, TcpStream};

use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // A single stream represents an open connection between the client and
        // the server. A connection is the name for the full request and
        // response process in which a client connects to the server, the
        // server generates a response, and the server closes the connection.

        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    // We’ve made the stream parameter mutable. The reason is that the
    // TcpStream instance keeps track of what data it returns to us internally.
    // It might read more data than we asked for and save that data for the
    // next time we ask for data. It therefore needs to be mut because its
    // internal state might change; usually, we think of “reading” as not
    // needing mutation, but in this case we need the mut keyword.

    // Next, we need to actually read from the stream. We do this in two steps:
    // first, we declare a buffer on the stack to hold the data that is read
    // in. We’ve made the buffer 1024 bytes in size, which is big enough to
    // hold the data of a basic request and sufficient for our purposes in this
    // chapter. If we wanted to handle requests of an arbitrary size, buffer
    // management would need to be more complicated;
    let mut buffer = [0; 1024];

    // read bytes from the TcpStream and put them in the buffer.
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // convert the bytes in the buffer to a string and print that string.
    // The String::from_utf8_lossy function takes a &[u8] and produces a String
    // from it. The “lossy” part of the name indicates the behavior of this
    // function when it sees an invalid UTF-8 sequence:
    // it will replace the invalid sequence with �, the U+FFFD REPLACEMENT
    // CHARACTER.
    // You might see replacement characters for characters in the buffer that
    // aren’t filled by request data.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
