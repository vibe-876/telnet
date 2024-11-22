//use std::io::prelude::*;
use std::net::*;


fn main() {
    println!("The address is {}.", build_address(Ipv4Addr::new(127,0,0,1), 20));
   // build_address(Ipv4Addr::new(127,0,0,1), 70);
}

/*
fn connect(address: Ipv4Addr, port: u16) -> u8 {
    let mut stream = TcpStream::connect(build_address(address, port));
    0
}
*/

fn build_address(address: Ipv4Addr, port: u16) -> String {
    let mut addr_and_port: String = address.to_string();

    addr_and_port.push(':');
    addr_and_port.push_str(&port.to_string());
    addr_and_port
}
