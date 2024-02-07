use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::u8;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "8080")]
    port: u16
}

fn main() {
    let args = Args::parse();
    let port = args.port;
    if let Ok(listener) = TcpListener::bind(format!("localhost:{port}")) {
        println!("Listening on port {port}");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => handle_stream(stream),
                Err(e) => eprintln!("{e}"),
            }
        }
    } else {
        eprintln!("Could not listen!");
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => print_buffer(&buffer),
        Err(e) => eprintln!("Error while reading from stream: {e}"),
    };
}

fn print_buffer(buffer: &[u8]) {
    let index_of_zero = buffer.iter().position(|x| *x == 0).unwrap_or(buffer.len());
    let (buffer_content, _) = buffer.split_at(index_of_zero);
    match std::str::from_utf8(buffer_content) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("Error while parsing buffer as utf8 string: {e}"),
    }
}
