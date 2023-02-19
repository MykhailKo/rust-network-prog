use getopt::Opt;

fn main() {
    // args = ["program", "-abc", "-d", "foo", "-e", "bar"];
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "n:p:");

    let addressSeparator: String = ":".to_owned();
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

    let textSockAddr: String = (nodename + &addressSeparator) + &port;

    println!("Translating address: {textSockAddr}")

}