use clap::{App, Arg};
use futures::{stream, StreamExt};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tokio::net::TcpStream;
use colored::*;

mod ports;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    banner();
    let cli_matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short("c")
                .default_value("1002"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Display detailed information")
                .long("verbose")
                .short("v"),
        )
        .arg(
            Arg::with_name("full")
                .help("Scan all 65535 ports")
                .long("full"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short("t")
                .default_value("3"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let full = cli_matches.is_present("full");
    let verbose = cli_matches.is_present("verbose");
    let concurrency = cli_matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1002);
    let timeout = cli_matches
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(3);
    let target = cli_matches.value_of("target").unwrap();

    if verbose {
        let ports = if full {
            String::from("All 65535 ports")
        } else {
            String::from("Most common (1002) ports")
        };
        println!("{}", "* Verbose mode is on".green().bold());
        println!(
            "{} {}",
            "* Hostname".green().bold(),
            format!("{}", target).yellow().bold()
        );
        println!(
            "{} {}",
            "* Scan Mode".green().bold(),
            format!("{}", &ports).yellow().bold()
        );
        println!(
            "{} {}",
            "* Concurrency".green().bold(),
            format!("{}", concurrency).yellow().bold()
        );
        println!(
            "{} {}",
            "* Timeout".green().bold(),
            format!("{} seconds", timeout).yellow().bold()
        );
    } else {
        let ports = if full {
            String::from("All 65535 ports")
        } else {
            String::from("Most common (1002) ports")
        };
        println!("{}", "* Verbose mode is off".red().bold());
        println!(
            "{} {}",
            "* Hostname".green().bold(),
            format!("{}", target).yellow().bold()
        );
        println!(
            "{} {}",
            "* Scan Mode".green().bold(),
            format!("{}", &ports).yellow().bold()
        );
    }
    println!("{}\n", "* ?????????(???_???)????????? - Starting scan - ?????????(???_???)?????????".green().bold());
    let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();

    if socket_addresses.is_empty() {
        return Err(anyhow::anyhow!("Socket_addresses list is empty"));
    }
    scan(socket_addresses[0].ip(), full, concurrency, timeout).await;

    Ok(())
}

async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => println!("{} {}", format!("{}:", port).blue(), "open".cyan()),
        _ => {}
    }
}

fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}


fn banner(){
        let banner: ColoredString = "
        ,---.
        CO-|| ??-(??`??.??????)->MEME32<-(??`??.??????)-??
        O  || |
        |   D J
       // /''''
      `'`'
     ---
        ".green();
        let xxx: ColoredString = "Sterben ist nicht so schlimm, wie es zu sterben scheint...!".magenta();
        println!("{} {}", banner, xxx);
        println!("{}", "---------------------------------------------------------------------->".yellow());
        println!("\n\t{} {}", "p4rti".green(), "- A simple port scanner written in Rust...".cyan());
        println!("\n\t\t{} : {} ", "Author".green(), "sc4rfurry".yellow());
        println!("\t\t{} : {} ", "Version".green(), "0.1".blue());
        println!("\t\t{} : {} ", "License".green(), "MIT".blue());
        println!("\t\t{} : {} ", "Github".green(), "https://github.com/sc4rfurry".blue());
        println!("{}", "---------------------------------------------------------------------->\n".yellow());
}