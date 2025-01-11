use std::io::{BufRead, BufReader, prelude::*};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::fs;
use std::path::Path;
use std::env;
use std::env::current_dir;

fn main() {
    // Use 7878 as it shouldn't conflict with anything else running locally
    println!("Listening for connections...");
    println!("{:?}", current_dir());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Attempt Received!");
        process_request(stream);
    }
}

fn process_request(mut stream: TcpStream) {
    // Request can be quite large so read the contents in a bufReader to remove read overhead
    let buffer = BufReader::new(&stream);

    // Need to only take lines that are valid UTF-8 otherwise we panic. (empty is not UTF-8)
    let request = buffer.lines().next().unwrap().unwrap();

    let important_elements = request.split(" ").collect::<Vec<&str>>();

    let request_type = important_elements[0].to_string();
    let resource = important_elements[1].to_string();
    let protocol = important_elements[2].to_string();

    let mut environment_dir = current_dir().unwrap().as_path().to_str().unwrap().to_string();
    let file_path_string = environment_dir + "\\src\\" + &resource[1..];
    let file_path = Path::new(&file_path_string);
    println!("{:?}", resource);
    println!("{:?}", file_path);

    //let mut response;

    let response;
    // Generate a response using the standard http format
    if file_path.exists(){
        let file_contents = fs::read_to_string(file_path).unwrap();
        response = generate_success_response(file_contents);
    }
    else{
        response = generate_not_found_response();
    }
    println!("{response}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn generate_success_response(file_contents: String) -> String {
    // Not much functionality for now
    let length = file_contents.chars().count();
    let status_line = "HTTP/1.1 200 OK";
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{file_contents}")
}

fn generate_not_found_response() -> String {
    let status_line = "HTTP/1.1 404 Not Found";
    format!("{status_line}")
}