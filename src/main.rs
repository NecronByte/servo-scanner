use clap::Parser;
use colored::*;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

/// ServoScanner: Scan ports with cult-approved precision.
#[derive(Parser, Debug)]
#[command(name = "servo-scanner")]
#[command(about = "Terminal-based TCP port scanner with NecronByte vibes.", long_about = None)]
struct Args {
    /// Target IP or hostname
    target: String,

    /// Ports to scan, e.g. 22,80,443 or 1-1024
    #[arg(short, long, default_value = "22,80,443")]
    ports: String,
}

fn scan_port(ip: &str, port: u16) -> bool {
    let addr_str = format!("{}:{}", ip, port);
    match addr_str.parse::<SocketAddr>() {
        Ok(addr) => TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok(),
        Err(_) => {
            eprintln!("{} Invalid address format: {}", "⚠️".yellow(), addr_str);
            false
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{}", "Servo-Scanner v0.1".green().bold());

    let ports: Vec<u16> = args.ports
        .split(',')
        .flat_map(|s| {
            if s.contains('-') {
                let parts: Vec<u16> = s.split('-')
                    .filter_map(|x| x.parse().ok())
                    .collect();
                if parts.len() == 2 {
                    (parts[0]..=parts[1]).collect()
                } else {
                    vec![]
                }
            } else {
                s.parse().ok().into_iter().collect()
            }
        })
        .collect();

    if ports.is_empty() {
        eprintln!("{}", "No valid ports specified.".red());
        return;
    }

    for port in ports {
        if scan_port(&args.target, port) {
            println!("{} {}", format!("Port {}:", port).cyan(), "OPEN".green());
        } else {
            println!("{} {}", format!("Port {}:", port).cyan(), "closed".red());
        }
    }
}