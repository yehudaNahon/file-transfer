extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
use clap::{App, SubCommand};
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Error};

fn listen(ip: str, port: u32) -> Result<(), Error> {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}



fn main() {
    env_logger::init();

    let matches = App::new("File Transfer")
        .version("0.0")
        .author("Yehuda Nahon <yehudanahon98@gmail.com>")
        .about("Transfers Files from server to client using a custom protocol")
        .arg_from_usage("-v, --verbose 'Sets the level of verbosity'")
        .subcommand(
            SubCommand::with_name("receive")
                .about("receives a file from server")
                .args_from_usage(
                    "<ip> 'The servers ip'
                    <port> 'The servers port'
                    <src> 'the remote file location'
                    <dst> 'the location to save the file in local machine'",
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("receive") => println!("receive was used"),
        _ => error!("Invalid command"),
    }
}
