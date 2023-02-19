use std::net::{ToSocketAddrs, SocketAddr, SocketAddrV4, SocketAddrV6};
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
                if ip_version == 0 {
                    println!("address: {address}, port: {port}");
                    continue;
                }
                match sock {
                    SocketAddr(SocketAddrV4 { ip, port }) => {
                        if ip_version == 4 {
                            println!("address: {address}, port: {port}");
                        }
                    },
                    SocketAddr(SocketAddrV6 { ip, port, flowinfo, scope_id }) => {
                        if ip_version == 6 {
                            println!("address: {address}, port: {port}");
                        }
                    }
                }
            }
        }
    }

}