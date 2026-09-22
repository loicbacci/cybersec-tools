use std::{env, net::TcpStream};

struct Args {
    host: String,
    port_start: u16,
    port_end: u16,
}

fn parse_args(args: &Vec<String>) -> Args {
    // Check arguments number
    // app + host + port start + port end => 4
    if args.len() != 4 {
        panic!("Wrong number of arguments. Expected \"portscan <host> <start-port> <end-port>\"")
    }

    // Parse arguments
    let host = args[1].clone();
    let port_start: u16 = args[2].parse().expect("port start should be a number");
    let port_end: u16 = args[3].parse().expect("port end should be a number");

    Args {
        host,
        port_start,
        port_end,
    }
}

fn scan_port(host: &str, port: &u16) {
    let addr = format!("{host}:{port}");

    match TcpStream::connect(addr) {
        Ok(_) => println!("{port}\tOPEN"),
        _ => println!("{port}\tCLOSED"),
    };
}

fn main() {
    println!("=== Port Scanner ===");

    // Parse arguments
    let prog_args: Vec<String> = env::args().collect();
    let args = parse_args(&prog_args);
    let Args {
        host,
        port_start,
        port_end,
    } = args;

    // Print args
    println!("Host       = {host}");
    println!("Port start = {port_start}");
    println!("Port end   = {port_end}");

    println!("\nScanning ports:");

    // Try connections
    for port in port_start..=port_end {
        scan_port(&host, &port);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_fails_on_wrong_number_of_args() {
        let incorrect = vec!["port-scan", "127.0.0.1", "1"];
        let args = incorrect
            .iter()
            .map(|v| String::from(*v))
            .collect::<Vec<String>>();

        parse_args(&args);
    }
}
