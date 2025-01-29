// use std::io::prelude::*;
use std::env;
use std::net::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let ip: Ipv4Addr = args[1].parse().expect("ip is in an invalid form.");
    let port: u16 = args[2].parse().unwrap();

    println!("ip is {ip}, and the port is {port}.");
    println!("The address is {}.", build_address(Ipv4Addr::new(127,0,0,1), 20));
}


fn build_address(address: Ipv4Addr, port: u16) -> String {
    let mut addr_and_port: String = address.to_string();

    addr_and_port.push(':');
    addr_and_port.push_str(&port.to_string());
    addr_and_port
}
