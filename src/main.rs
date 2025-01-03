use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
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
        let stream = stream.unwrap();
        // TODO: TcpListener bind returns a result. We need failure handling instead of just unwrapping
        println!("Connection established!");
        let requested_resource = process_request(stream);
        get_file_if_exists()
    }
}

fn get_file_if_exists() {
    todo!()
}

fn process_request(stream: TcpStream) -> String {
    // Function to return the resource requested

    // Request can be quite large so read the contents in a bufReader to remove read overhead
    let buffer = BufReader::new(&stream);

    // Need to only take lines that are valid UTF-8 otherwise we panic. (empty is not UTF-8)
    let request: Vec<_>  = buffer.lines().map(|line| line.unwrap()).take_while(|content| !content.is_empty()).collect();

    // Get the request type, resource and protocol
    let important_request_information = request[0].clone();
    let important_elements = important_request_information.split(" ").collect::<Vec<&str>>();

    let request_type = important_elements[0];
    let resource = important_elements[0];
    let protocol = important_elements[0];

    String::from(resource)


}
