pub mod config {
    use std::{fs, path::PathBuf};

    #[allow(dead_code)]
    pub fn read_stored_priv_key() -> String {
        fs::read_to_string(get_config_file()).expect("Couldn't read lainapps config")
    }

    pub fn get_config_dir() -> PathBuf {
        match dirs::config_dir() {
            Some(c) => c.join("lainapps"),
            None => panic!("Couldn't get config dir")
        }
    }

    pub fn get_config_file() -> PathBuf { get_config_dir().join("gitwrapper.config") }
}
