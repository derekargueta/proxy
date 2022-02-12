use std::io::{self, BufRead, Write};
use std::net::TcpListener;
use std::net::TcpStream;

use clap::{Arg, App};

fn tcp_proxy(source: &str, destination: &str) -> io::Result<()> {
    let listener = TcpListener::bind(source).unwrap();
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("new client: {:?}", addr);
            let mut reader = io::BufReader::new(&stream);
            let mut stream = TcpStream::connect(destination)?;

            loop {
                let bytes = reader.fill_buf()?;
                let len = bytes.len();

                stream.write(bytes);
                stream.flush()?;

                reader.consume(len);
                println!("proxied {:?} bytes", len);

                // TODO - better mechanism of detecting broken connection.
                if len == 0 {
                    break Ok(());
                }
            }
        },
        Err(e) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("couldn't get client: {:?}", e)
        ))
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("Proxy")
        .version("0.0.1")
        .author("Derek Argueta")
        .arg(Arg::new("source")
                 .short('s')
                 .long("source")
                 .takes_value(true)
                 .help("source address"))
        .arg(Arg::new("destination")
                 .short('d')
                 .long("destination")
                 .takes_value(true)
                 .help("destination address"))
        .get_matches();
    let source = matches.value_of("source").unwrap_or("127.0.0.1:8080");
    let destination = matches.value_of("destination").unwrap_or("127.0.0.1:9000");

    loop {
        tcp_proxy(source, destination);
    }

    Ok(())
}
