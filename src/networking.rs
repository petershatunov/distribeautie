use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::{config, storage};

pub(crate) fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line_optional = buf_reader.lines().next();
    let request_line = if request_line_optional.is_some() {
        request_line_optional.unwrap().unwrap()
    } else {
        return;
    };

    println!("Request: {:#?}", request_line);

    if request_line.starts_with("POST") {
        write_path(request_line.clone(), stream.try_clone().unwrap());
    }
    if request_line.starts_with("GET") {
        read_path(request_line.clone(), stream.try_clone().unwrap());
    }
}

fn read_path(get_request_line: String, mut stream: TcpStream) {
    let object_to_read = get_object(get_request_line);
    let item_exists = storage::get_item(object_to_read.clone());

    println!(
        "object to read: {:#?}, item exists: {:#?}",
        object_to_read, item_exists
    );

    let status_line = "HTTP/1.1 200 OK";
    let contents = "The following object exists: ".to_owned() + &item_exists.to_string();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn write_path(write_access_method: String, mut stream: TcpStream) {
    let object_to_create = get_object(write_access_method);

    println!("object to create: {:#?}", object_to_create);

    storage::add_item(object_to_create.clone());

    let status_line = "HTTP/1.1 201 DISTRIBEAUTIE_OBJECT_CREATED";
    let contents = format!("The following object has been created: {object_to_create}");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_object(request_line: String) -> String {
    request_line.split_whitespace().collect::<Vec<_>>()[1].replace("/", "")
}

pub(crate) fn update_replicas() {
    let replicas = config::get_replicas_addr();
    let all_items = storage::get_all_items();

    for replica_addr in replicas {
        match TcpStream::connect(replica_addr.clone()) {
            Ok(mut stream) => {
                for item in &all_items {
                    let body = "key1=value1&key2=value2";
                    let request = format!(
                        "POST /{} HTTP/1.1\r\n\
                    Host: localhost\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: {}\r\n\
                    \r\n\
                    {}",
                        item,
                        body.len(),
                        body
                    );
                    // Send the request to a replica
                    stream
                        .write_all(request.as_bytes())
                        .expect("Failed to write to stream");
                    let mut response = String::new();
                    stream
                        .read_to_string(&mut response)
                        .expect("Failed to read from stream");

                    println!("Response: {}", response);
                }
            }
            Err(e) => {
                println!("Failed to connect: {} {}", replica_addr, e);
            }
        }
    }
}
