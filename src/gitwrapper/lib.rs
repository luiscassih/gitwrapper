pub mod config {
    use std::{fs, io, path::PathBuf};

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfigYaml {
        pub priv_key: String,
        pub ssh_bin: String,
    }

    pub fn get_config_yaml() -> ConfigYaml {
        let default_config = ConfigYaml {
            priv_key: "".to_string(),
            ssh_bin: "gitwrapper-ssh".to_string(),
        };

        match fs::read_to_string(get_config_file()) {
            Ok(f) => match serde_yaml::from_str(&f) {
                Ok(s) => s,
                Err(_) => default_config,
            },
            Err(_) => {
                fs::create_dir_all(get_config_dir()).expect("Couldn't create config directory.");
                fs::write(
                    get_config_file(),
                    serde_yaml::to_string(&default_config).unwrap(),
                )
                .expect("Couldn't create default config.");
                default_config
            }
        }
    }

    pub fn get_config_dir() -> PathBuf {
        match dirs::config_dir() {
            Some(c) => c.join("gitwrapper"),
            None => panic!("Couldn't get config dir"),
        }
    }

    pub fn get_config_file() -> PathBuf {
        get_config_dir().join("gitwrapper.config")
    }

    #[allow(dead_code)]
    pub fn save_config(priv_key: Option<&PathBuf>, ssh_bin: Option<String>) -> io::Result<()> {
        let mut config_yaml = get_config_yaml();
        if let Some(p) = priv_key {
            let priv_key_path = fs::canonicalize(p)?;
            config_yaml.priv_key = priv_key_path.display().to_string();
        }
        if let Some(s) = ssh_bin {
            config_yaml.ssh_bin = s;
        }
        fs::write(
            get_config_file(),
            serde_yaml::to_string(&config_yaml).unwrap(),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::config::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn create_config() {
        let canonicalized_file = fs::canonicalize(&PathBuf::from("/bin/sh".to_string()))
            .expect("Error canonicalizing path.");
        save_config(Some(&canonicalized_file), None).expect("Error on creating config file");
        assert_eq!(
            get_config_yaml().priv_key,
            canonicalized_file.display().to_string()
        );
    }
}
