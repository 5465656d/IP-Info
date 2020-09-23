// File - src/main.rs
use ipinfo;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;

fn main() {

    // TODO - handle arg parsing in better way, probably with structopt or clap
    let args: Vec<String> = std::env::args().collect();
    let ip_from_arg = &args[2];
    
    let ip: IpAddr = IpAddr::from_str(&ip_from_arg).unwrap_or_else(|err| {
        println!("Problem searching for IP: {}: {}", ip_from_arg, err);
        process::exit(1);
    });
    
    println!("Searching for {} ... ", ip);
    let ip_data = ipinfo::ip_lookup(&ip).unwrap_or_else(|err| {
        // error message is specific to inside Err()
        println!("Problem searching for IP: {}: {}", ip, err);
        process::exit(1);
    });

    // debug - print struct
    println!("{:#?}", ip_data);
}
