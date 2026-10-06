use clap::{Parser, Subcommand};

use crate::port_scan::{PortScanArgs, port_scan};

mod port_scan;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Port scan
    PortScan(PortScanArgs),
}

fn main() {
    let cli = Cli::parse();

    // Find which subcommand
    match &cli.command {
        Commands::PortScan(args) => {
            port_scan(args);
        }
    }
}
