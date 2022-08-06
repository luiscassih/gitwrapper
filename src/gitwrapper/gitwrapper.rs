use std::{io, fs, path::PathBuf, process::{Command, ExitStatus}};
use clap::{Parser, Subcommand};
mod lib;

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
}

fn main() {
    let args = Cli::parse();
    match args.command {
        CliCommands::Set { priv_key } => {
            set_priv_key(&priv_key).expect("Invalid priv key");
        },
        CliCommands::Git { git_args } => {
            if let Err(e) = call_git_command(&git_args) {
                println!("Couldn't execute git command: {}", e)
            }
        },
    }
}

fn set_priv_key(priv_key: &PathBuf) -> Result<(), io::Error> {
    fs::create_dir_all(lib::config::get_config_dir())?;
    let priv_key_path = fs::canonicalize(priv_key)?;
    println!("Creating file with this key: {:?}", priv_key_path);
    fs::write(lib::config::get_config_file(), priv_key_path.display().to_string())?;
    Ok(())
}

fn call_git_command(args: &Vec<String>) -> io::Result<ExitStatus> {
    Command::new("git").env("GIT_SSH", "gitwrapper-ssh").args(args).spawn()?.wait()
}
