use std::{io, fs, path::PathBuf, process::{Command, ExitStatus}};

use clap::{Parser, Subcommand};

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

    /// A proxy to ssh wrapped with your private key
    Ssh {
        #[clap(multiple=true, allow_hyphen_values = true)]
        ssh_args: Vec<String>,
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
        CliCommands::Ssh { ssh_args } => {
            if let Err(e) = call_ssh_command(&ssh_args) {
                println!("Couldn't execute ssh command: {}", e)
            }
        }
    }
}

fn read_stored_priv_key() -> String {
    fs::read_to_string(get_config_file()).expect("Couldn't read lainapps config")
}

fn get_config_dir() -> PathBuf {
    match dirs::config_dir() {
        Some(c) => c.join("lainapps"),
        None => panic!("Couldn't get config dir")
    }
}

fn get_config_file() -> PathBuf { get_config_dir().join("gitwrapper.config") }

fn set_priv_key(priv_key: &PathBuf) -> Result<(), io::Error> {
    fs::create_dir_all(get_config_dir())?;
    let priv_key_path = fs::canonicalize(priv_key)?;
    println!("Creating file with this key: {:?}", priv_key_path);
    fs::write(get_config_file(), priv_key_path.display().to_string())?;
    Ok(())
}

fn call_git_command(args: &Vec<String>) -> io::Result<ExitStatus> {
    Command::new("git").env("GIT_SSH", "gitwrapper ssh").args(args).spawn()?.wait()
}

fn call_ssh_command(args: &Vec<String>) -> io::Result<ExitStatus> {
    let mut joined_args = vec!["-i".to_string(), read_stored_priv_key()];
    joined_args.extend(args.to_owned());
    Command::new("ssh").args(joined_args).spawn()?.wait()
}
