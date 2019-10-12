extern crate clap;
use clap::{App, SubCommand};

fn main() {
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

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
    match matches.subcommand_name() {
        Some("receive") => println!("receive was used"),
        _ => println!("Invalid command"),
    }
}
