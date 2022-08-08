use std::{io, fs, path::PathBuf, process::{Command, ExitStatus}};
use clap::{Parser, Subcommand};
mod lib;
use lib::config::*;

/// A git wrapper to use your own personal private ssh key
#[derive(Parser, Debug)]
struct Cli {
    /// Private ssh key
    #[clap(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand, Debug)]
enum CliCommands {
    /// Set your private ssh key to use in this wrapper
    Set {
        /// Path to your desired private ssh key
        priv_key: PathBuf,
    },

    /// A proxy to git command but wrapped with your configured private key
    Git {
        #[clap(multiple=true, allow_hyphen_values = true)]
        git_args: Vec<String>,
    },

    /// View configured private ssh key
    View { },

    // Clear configured key
    Clear { },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        CliCommands::Set { priv_key } => {
            set_priv_key(&priv_key).expect("Invalid priv key");
            println!("Config file successfully created.");
        },
        CliCommands::Git { git_args } => {
            let config_yaml: ConfigYaml = get_config_yaml();
            assert!(!config_yaml.priv_key.is_empty(), "Private key in config is empty, please use Set subcommand to configure a key.");
            if let Err(e) = call_git_command(&git_args) {
                println!("Couldn't execute git command: {}", e)
            }
        },
        CliCommands::View {  } => {
            let config_yaml = get_config_yaml();
            println!("Configured settings:\nPrivate key path: {}\nWrapped ssh binary: {}", config_yaml.priv_key, config_yaml.ssh_bin);
        },
        CliCommands::Clear {  } => {
            match fs::remove_file(get_config_file()) {
                Ok(()) => println!("Successfully removed {:?}", get_config_file()),
                Err(e) => println!("Couldn't remove config file. {}", e),
            }
        },
    }
}

fn call_git_command(args: &Vec<String>) -> io::Result<ExitStatus> {
    let config_yaml = get_config_yaml();
    Command::new("git").env("GIT_SSH", config_yaml.ssh_bin).args(args).spawn()?.wait()
}
