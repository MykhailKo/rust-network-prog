use std::net::{SocketAddr, ToSocketAddrs};
use getopt::Opt;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "n:p:");

    let address_separator: String = ":".to_owned();
    let mut nodename = String::new();
    let mut port = String::new();

    loop {
        match opts.next().transpose() {
            Ok(None) => break,
            Ok(Some(opt)) => match opt {
                Opt('n', Some(arg)) => nodename = arg.clone(),
                Opt('p', Some(arg)) => port = arg.clone(),
                _ => unreachable!(),
            },
            Err(_) => println!("Invalid input")
        }
    }

    let text_sock_addr: String = (nodename + &address_separator) + &port;
    println!("Translating address: {text_sock_addr}")

    let sock_addresses = text_sock_addr.to_socket_addrs().unwrap();

    loop {
        match sock_addresses.next().transpose() {
            Ok(None) => break,
            Ok(Some(sock)) => println!("address: {sock.ip().to_string()}, port: {sock.port().to_string()}")
        }
    }


}