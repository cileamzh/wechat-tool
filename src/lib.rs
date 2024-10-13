use std::{
    collections::HashMap,
    env::{current_dir, Args},
    ffi::OsStr,
    fs,
    path::PathBuf,
    process::Command,
};

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

// excute the program with a path
pub fn excute<I, S>(path: String, args: I) -> std::io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(path).args(args).spawn()?;
    Ok(())
}

// you commandline instruction will considered as a task
pub struct Task {
    pub order: String,
    pub params: HashMap<String, String>,
}

impl Task {
    pub fn from(mut args: Args) -> Task {
        let mut al: i32 = args.len() as i32;
        args.next();
        let order = args.next().unwrap_or("noins".to_owned());
        let mut map = HashMap::new();
        al = al - 2;
        loop {
            if al <= 0 {
                break;
            }
            map.insert(
                args.next().unwrap_or("none".to_owned()),
                args.next().unwrap_or("none".to_owned()),
            );
            al = al - 2;
        }

        Task {
            order: order,
            params: map,
        }
    }
}

// Excutor is a tool to excute your task
pub struct Excutor {
    pub config: Config,
    cdir: PathBuf,
}

impl Excutor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cdir: current_dir().unwrap(),
        }
    }
    pub fn exe_task(&mut self, task: Task) -> std::io::Result<()> {
        match task.order.as_str() {
            "run" => {
                excute(
                    self.config.clipath.clone(),
                    ["open", "--project", &self.cdir.to_string_lossy()],
                )?;
            }
            "preview" => {
                excute(
                    self.config.clipath.clone(),
                    [
                        "preview",
                        "--project",
                        &self.cdir.to_string_lossy(),
                        "--qr-size",
                        "small",
                    ],
                )?;
            }
            "close" => {
                excute(
                    self.config.clipath.clone(),
                    ["close", "--project", &self.cdir.to_string_lossy()],
                )?;
            }
            "quit" => {
                excute(self.config.clipath.clone(), ["quit"])?;
            }
            "login" => {
                excute(self.config.clipath.clone(), ["login"])?;
            }
            "islogin" => {
                excute(self.config.clipath.clone(), ["islogin"])?;
            }
            "cdir" => {
                println!("{}", self.cdir.display())
            }
            "help" => {
                println!("{}", HELP);
            }
            // get the props
            "set" => {
                for (k, v) in task.params.iter() {
                    match k.as_str() {
                        "devtoolpath" => {
                            self.config.binpath = v.to_owned();
                            fs::write(self.config.cfgpath.clone(), self.config.to_binary())?;
                        }
                        _ => {}
                    }
                }
            }
            "get" => {
                for k in task.params.keys() {
                    match k.as_str() {
                        "devtoolpath" => {
                            println!("{}", self.config.binpath)
                        }
                        "configpath" => {
                            println!("{}", self.config.cfgpath.display())
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// This is the help document
// Get this doc by inputing help
static HELP: &str = "
run : run your app in WechatDeveloperTool.
config :change or get your setting infomation
";
