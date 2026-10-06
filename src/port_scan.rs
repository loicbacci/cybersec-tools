use clap::Args;
use std::net::TcpStream;

type Port = u16;

#[derive(Args)]
pub struct PortScanArgs {
    #[arg(long)]
    host: String,

    #[arg(long)]
    #[arg(value_parser = clap::value_parser!(Port).range(1..))]
    port_start: Port,

    #[arg(long)]
    #[arg(value_parser = clap::value_parser!(Port).range(1..))]
    port_end: Port,
}

/// Checks if the given ports are open on the host.
///
/// `port_end` is included.
pub fn port_scan(args: &PortScanArgs) {
    let host = &args.host;
    let start = &args.port_start;
    let end = &args.port_end;

    // Validate
    if end < start {
        eprintln!("End port {end} is before start {start}");
        std::process::exit(2);
    }

    // Try connections
    for port in args.port_start..=args.port_end {
        let addr = format!("{host}:{port}");

        match TcpStream::connect(addr) {
            Ok(_) => println!("{port}\tOPEN"),
            _ => println!("{port}\tCLOSED"),
        };
    }
}
