extern crate fizzbuzz;

use fizzbuzz::fizzbuzz_number_formatter::FizzbuzzMessageFormatter;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

///
/// A simple reference implementation using the fizzbuzz library
///
fn main() {

    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    let response = generate_response(&stream);
                    let mut writer = BufWriter::new(stream);
                    writer.write_all(response.as_bytes()).unwrap();
                });
            }
            Err(_) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}

fn generate_response(stream: &TcpStream) -> String {

    let formatter = FizzbuzzMessageFormatter::default();

    let mut reader = BufReader::new(stream);

    let request_line = parse_request_line(&mut reader);
    let http_headers = parse_http_header(&mut reader);

    let tokens: Vec<&str> = request_line.split(' ').collect();
    let result = tokens[1].replace("/", "").parse::<i64>();
    let mut content = "".to_string();
    let mut response = "".to_string();

    match result {
        Ok(result) => {
            content = formatter.format_number(result);
        }
        Err(_) => {
            content = "error processing request".to_string();
        }
    }

    response = generate_response_headers(&content);
    response.push_str(&content);

    response
}

fn generate_response_headers(content: &str) -> String {

    let mut headers = "HTTP/1.1 200 OK\r\n".to_string();

    headers.push_str("Connection: Closed\r\n");
    headers.push_str("Content-Type: text/html\r\n");
    headers.push_str("Server: Rust\r\n");
    headers.push_str("Content-Length: ");
    headers.push_str(&(content.as_bytes().len().to_string()));
    headers.push_str("\r\n\r\n");

    headers
}

fn parse_request_line(reader: &mut BufReader<&TcpStream>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line
}

fn parse_http_header(reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {

    let mut headers : HashMap<String, String> = HashMap::new();

    // Read all the http headers
    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);
        match result {
            Ok(_) => {
                // The last line of an http header is blank
                if line.trim().is_empty() {break;}

                // ':' separates header keys from values
                let tokens: Vec<&str> = line.split(':').collect();

                headers.insert(String::from(tokens[0].trim()), String::from(tokens[1].trim()));
            }
            Err(_) => { /* An error occurred */ }
        }
    }

    headers
}
