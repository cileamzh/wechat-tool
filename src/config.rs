use std::{fs, path::PathBuf};

//Function get_config is used to check and get the config
pub fn get_config(dir: PathBuf, file: PathBuf) -> std::io::Result<Config> {
    if !dir.join(&file).exists() {
        fs::File::create(&dir.join(&file))?;
        Ok(Config::open(dir.join(file)))
    } else {
        Ok(Config::open(dir.join(file)))
    }
}

// struct config represent the Excutor config
pub struct Config {
    pub cfgpath: PathBuf,
    pub binpath: String,
    pub clipath: String,
}

impl Config {
    pub fn open(path: PathBuf) -> Self {
        let mut devtoolpath = String::new();
        let config = fs::read(path.clone()).unwrap();
        let config = String::from_utf8_lossy(&config);
        for line in config.split("/r/n") {
            let s: Vec<&str> = line.split("=").collect();
            match s[0] {
                "devtoolpath" => devtoolpath = s[1].to_owned(),
                _ => {}
            }
        }
        Config {
            cfgpath: path,
            binpath: devtoolpath.clone(),
            clipath: format!("{}\\cli.bat", devtoolpath),
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let config = format!("devtoolpath={}", self.binpath);
        config.as_bytes().to_vec()
    }
}
