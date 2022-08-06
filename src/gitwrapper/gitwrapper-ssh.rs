use std::{io, process::{Command, ExitStatus}};

use clap::Parser;
// #[path = "gitwrapper.rs"] mod gitwrapper;
mod lib;

/// A git wrapper to use your own personal private ssh key
#[derive(Parser, Debug)]
struct Cli {
    /// Private ssh key
    #[clap(multiple=true, allow_hyphen_values = true)]
    ssh_args: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = call_ssh_command(&args.ssh_args) {
        println!("Couldn't execute ssh command: {}", e)
    }
}

fn call_ssh_command(args: &Vec<String>) -> io::Result<ExitStatus> {
    let mut joined_args = vec!["-i".to_string(), lib::config::read_stored_priv_key()];
    joined_args.extend(args.to_owned());
    Command::new("ssh").args(joined_args).spawn()?.wait()
}

