use std::net::{ToSocketAddrs};
use getopt::Opt;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "n:p:v");

    let address_separator: String = ":".to_owned();
    let mut nodename = String::new();
    let mut port = String::new();
    let mut ip_version = 0;

    loop {
        match opts.next().transpose() {
            Ok(None) => break,
            Ok(Some(opt)) => match opt {
                Opt('n', Some(arg)) => nodename = arg.clone(),
                Opt('p', Some(arg)) => port = arg.clone(),
                Opt('v', Some(arg)) => ip_version = arg.clone().trim().parse().unwrap(),
                _ => unreachable!(),
            },
            Err(_) => println!("Invalid input")
        }
    }

    let text_sock_addr: String = (nodename + &address_separator) + &port;
    println!("Translating address: {text_sock_addr}");

    let mut sock_addresses = text_sock_addr.to_socket_addrs().unwrap();

    loop {
        match sock_addresses.next() {
            None => break,
            Some(sock) => {
                let address = sock.ip().to_string();
                let port = sock.port().to_string();
                } if ip_version == 4 && sock.is_ipv4() {
                    println!("address: {address}, port: {port}");
                } else if ip_version == 6 && sock.is_ipv6() {
                    println!("address: {address}, port: {port}");
                } else {
                    println!("address: {address}, port: {port}");
                }
            }
        }
    }

}