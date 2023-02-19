use getopt::Opt;

fn main() {
    // args = ["program", "-abc", "-d", "foo", "-e", "bar"];
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "nodename:port:");

    let addressSeparator: String = ":".to_owned();
    let mut nodename = String::new();
    let mut port = String::new();

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt("nodename", Some(arg)) => nodename = arg.clone(),
                Opt("port", Some(arg)) => nodename = arg.clone(),
                _ => unreachable!(),
            },
        }
    }

    let textSockAddr: String = (nodename + &addressSeparator) + &port;

    println!("Translating address: {textSockAddr}")

}