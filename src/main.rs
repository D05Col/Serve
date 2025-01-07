use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::Path;

fn main() {
    // Use 7878 as it shouldn't conflict with anything else running locally
    println!("Listening for connections...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    handle_connection(listener);
}

fn handle_connection(listener: TcpListener) {
    // Iterating the streams like this is synchronous for now
    for stream in listener.incoming() {
        // The way that the tcp listener works is it queues connection attempts trying to connect
        // This tcp stream will be open to read the request and write the response following HTTP formats
        let stream = stream.unwrap();
        // TODO: TcpListener bind returns a result. We need failure handling instead of just unwrapping
        println!("Connection established!");
        process_request(stream);

    }
}

fn process_request(mut stream: TcpStream) {
    // Function to return the resource requested

    // Request can be quite large so read the contents in a bufReader to remove read overhead
    let buffer = BufReader::new(&stream);

    // Need to only take lines that are valid UTF-8 otherwise we panic. (empty is not UTF-8)
    let request: Vec<_>  = buffer.lines().map(|line| line.unwrap()).collect();//.take_while(|content| !content.is_empty()).collect();

    // Get the request type, resource and protocol
    let important_request_information = request[0].clone();
    let important_elements = important_request_information.split(" ").collect::<Vec<&str>>();

    let request_type = important_elements[0];
    let resource = important_elements[0];
    let protocol = important_elements[0];

    let file = Path::new(resource);

    let mut response;
    // Generate a response using the standard http format
    if file.exists(){
        let file_contents = fs::read_to_string(file).unwrap();
        response = generate_success_response(file_contents);
    }
    else{
        response = generate_not_found_response();
    }
    stream.write_all(response.as_bytes()).unwrap();
}

fn generate_success_response(file_contents: String) -> String {
    // Not much functionality for now
    let length = file_contents.chars().count();
    let status_line = "HTTP/1.1 200 OK";
    String::from(format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{file_contents}"))
}

fn generate_not_found_response() -> String {
    let status_line = "HTTP/1.1 404 Not Found";
    String::from(format!("{status_line}"))
}