use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
fn main() {
    establish_connection();
    //find_file();
    //generate_response()
}

fn establish_connection() {
    // Use 7878 as it shouldn't conflict with anything else running locally
    println!("Listening for connections...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // TODO: TcpListener bind returns a result. We need failure handling instead of just unwrapping
    for stream in listener.incoming() {
        // Iterating the streams like this is synchronous for now

        // I'm guessing the way that the tcp listener works is it queues connection attempts trying to connect
        // and deals with them one at a time
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    // Request can be quite large so read the contents in a bufReader to remove read overhead
    let buffer = BufReader::new(&stream);
    // Need to only take lines that are valid UTF-8 otherwise we panic. (empty is not UTF-8)
    let request_content: Vec<_>  = buffer.lines().map(|line| line.unwrap()).take_while(|content| !content.is_empty()).collect();
    println!("Request content: {request_content:#?}");
}
