pub mod config {
    use std::{fs, path::PathBuf, io};

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfigYaml {
        pub priv_key: String,
        pub ssh_bin: String,
    }

    pub fn get_config_yaml() -> ConfigYaml {
        let default_config = ConfigYaml {
            priv_key: "".to_string(),
            ssh_bin: "lainssh".to_string(),
        };

        match fs::read_to_string(get_config_file()) {
            Ok(f) => {
                match serde_yaml::from_str(&f) {
                    Ok(s) => s,
                    Err(_) => default_config
                }
            },
            Err(_) => {
                fs::create_dir_all(get_config_dir()).expect("Couldn't create config directory.");
                fs::write(get_config_file(), serde_yaml::to_string(&default_config).unwrap()).expect("Couldn't create default config.");
                default_config
            }
        }
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
        let priv_key_path = fs::canonicalize(priv_key)?;
        let mut config_yaml = get_config_yaml();
        config_yaml.priv_key = priv_key_path.display().to_string();
        fs::write(get_config_file(),serde_yaml::to_string(&config_yaml).unwrap())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs};
    use super::config::*;

    #[test]
    fn create_config() {
        let canonicalized_file = fs::canonicalize(&PathBuf::from("/bin/sh".to_string())).expect("Error canonicalizing path.");
        set_priv_key(&canonicalized_file).expect("Error on creating config file");
        assert_eq!(get_config_yaml().priv_key, canonicalized_file.display().to_string());
    }
}
