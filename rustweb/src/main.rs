use std::{net::TcpListener, str::Lines, io::{Read, Write}, thread::sleep, time::Duration};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000...");

    // let result = listener.accept().unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connection_events");

        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        println!("response from server:{:?}", std::str::from_utf8(&buffer).unwrap());

        sleep(Duration::from_secs(5));
        stream.write(&mut buffer).unwrap();
    }

}
