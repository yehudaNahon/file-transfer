extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
use env_logger::Builder;
use log::LevelFilter;

use clap::{App, SubCommand};
use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn listen(port: u16) -> Result<(), Error> {
    //TODO: add ip param and add in to the socket
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let listener = TcpListener::bind(socket)?;
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    debug!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    debug!("{:?} says {}", addr, input);
    Ok(())
}


fn main() {
    let matches = App::new("File Transfer")
        .version("0.0")
        .author("Yehuda Nahon <yehudanahon98@gmail.com>")
        .about("Transfers Files from server to client using a custom protocol")
        .arg_from_usage("-v... 'Sets the level of verbosity'")
        .subcommands(vec![
            SubCommand::with_name("receive")
                .about("receives a file from server")
                .args_from_usage(
                    "<ip> 'The servers ip'
                    <port> 'The servers port'
                    <src> 'the remote file location'
                    <dst> 'the location to save the file in local machine'",
                ),
            SubCommand::with_name("listen")
                .about("listen for data from socket")
                .arg_from_usage("<port> 'The servers port'"),
        ])
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 | _ => LevelFilter::Debug,
    };
    Builder::new().filter_level(log_level).init();

    //TODO: change to get port from args
    match matches.subcommand_name() {
        Some("receive") => println!("receive was used"),
        Some("listen") => match listen(8000) {
            Err(e) => error!("failed listening with error: {:?}", e),
            Ok(_) => {}
        },
        _ => error!("Invalid command"),
    }
}
