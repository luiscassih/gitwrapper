use std::{io, fs, path::Path};

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
    /// Set private key to use in this terminal
    Set {
        /// priv keeey
        priv_key: String,
    },
}

fn main() {
    let args = Cli::parse();
    println!("Hello, world!, {:?}", args);
    match args.command {
        CliCommands::Set { priv_key } => {
            set_priv_key(&priv_key).expect("Invalid priv key");
        }
    }
}

fn set_priv_key(priv_key: &str) -> Result<(), io::Error> {
    println!("Creating file: {}", priv_key);
    let config_dir = match dirs::config_dir() {
        Some(c) => c.join("lainapps"),
        None => panic!("Couldn't get config dir")
    };
    fs::create_dir_all(&config_dir)?;
    let priv_key_path = fs::canonicalize(Path::new(priv_key))?;
    fs::write(config_dir.join("gitwrapper.config"), priv_key_path.display().to_string() + "\n")?;
    
    // if priv_key == "asd" {
    //     return Err(io::Error::new(io::ErrorKind::NotFound, "Lol"));
    // };
    Ok(())
}
