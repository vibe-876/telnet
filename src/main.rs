#![allow(unused)]
use std::env;
use std::io;
use std::io::prelude::*;
use std::net::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let ip: Ipv4Addr = args[1].parse().expect("ip is in an invalid form.");
    let port: u16 = args[2].parse().unwrap();
    let full_address = build_address(ip, port);

    loop {
	let mut buffer = String::new();
	match io::stdin().read_line(&mut buffer) {
	    Ok(n) => {
		if(buffer == "exit\n".to_string()) { break };
		send_message(&buffer);
	    }
	    
	    Err(error) => println!("{error}"),
	}
    }
}


fn build_address(address: Ipv4Addr, port: u16) -> String {
    let mut addr_and_port: String = address.to_string();

    addr_and_port.push(':');
    addr_and_port.push_str(&port.to_string());
    addr_and_port
}

fn send_message(input: &String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(input)?;
    stream.write(input.as_bytes());
    Ok(())
}
