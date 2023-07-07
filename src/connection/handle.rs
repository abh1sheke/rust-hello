use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    process,
};

use log::{error, info, warn};

pub fn handle_connection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap_or_else(|| {
            error!("error occured while trying to read request");
            Ok(String::from("GET /404 HTTP/1.1"))
        })
        .unwrap_or_else(|err| {
            error!("error occured while trying to read request; {:#?}", err);
            warn!("shutting down.");
            process::exit(1);
        });
    info!("{:#?}", request_line);
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "web/index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "web/404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        error!("error occured while trying to read file; {:#?}", err);
        warn!("shutting down.");
        process::exit(1);
    });
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
