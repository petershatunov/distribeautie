use std::net::TcpListener;
use std::thread;

mod config;
mod networking;
mod scheduler;
mod storage;

fn main() {
    let server_addr = config::get_server_address();
    let is_masterhost = config::is_masterhost();
    println!(
        "Starting DistriBeautie server with the following config: {:#?} master: {:#?}",
        server_addr, is_masterhost
    );

    thread::spawn(|| scheduler::run());

    let listener = TcpListener::bind(server_addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| networking::handle_connection(stream));
    }
}
