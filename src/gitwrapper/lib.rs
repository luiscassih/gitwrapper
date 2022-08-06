pub mod config {
    use std::{fs, path::{PathBuf, Path}, io};

    pub fn read_stored_priv_key() -> String {
        let priv_key = fs::read_to_string(get_config_file()).expect("Couldn't read lainapps config");
        assert!(Path::new(&priv_key).exists());
        priv_key
    }

    pub fn get_config_dir() -> PathBuf {
        match dirs::config_dir() {
            Some(c) => c.join("lainapps"),
            None => panic!("Couldn't get config dir")
        }
    }

    pub fn get_config_file() -> PathBuf { get_config_dir().join("gitwrapper.config") }

    #[allow(dead_code)]
    pub fn set_priv_key(priv_key: &PathBuf) -> io::Result<()> {
        fs::create_dir_all(get_config_dir())?;
        let priv_key_path = fs::canonicalize(priv_key)?;
        println!("Creating file with this key: {:?}", priv_key_path);
        fs::write(get_config_file(), priv_key_path.display().to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::config::*;

    #[test]
    fn create_config() {
        set_priv_key(&PathBuf::from("/bin/sh".to_string())).expect("Error on creating config file");
        assert_eq!(read_stored_priv_key(), "/bin/sh");
    }
}
