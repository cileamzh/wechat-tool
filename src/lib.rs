use std::{
    collections::HashMap,
    env::{args, current_dir, current_exe, Args},
    ffi::OsStr,
    fs,
    io::{self},
    path::{Path, PathBuf},
    process::{Child, Command},
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
pub fn excute<I, S>(path: String, args: I) -> std::io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Ok(Command::new(path).args(args).spawn()?)
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
        let order = args.next().unwrap_or("".to_owned());
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
    pub rundir: PathBuf,
    pub exedir: PathBuf,
    pub temdir: PathBuf,
}

impl Excutor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            rundir: current_dir().unwrap(),
            exedir: current_exe().unwrap().parent().unwrap().to_path_buf(),
            temdir: current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf()
                .join("temp"),
        }
    }
    pub fn exe_task(&mut self, task: Task) -> std::io::Result<()> {
        match task.order.as_str() {
            "run" => {
                excute(
                    self.config.clipath.clone(),
                    ["open", "--project", &self.rundir.to_string_lossy()],
                )?
                .wait()?;
            }
            "preview" => {
                excute(
                    self.config.clipath.clone(),
                    [
                        "preview",
                        "--project",
                        &self.rundir.to_string_lossy(),
                        "--qr-size",
                        task.params.get("-size").unwrap_or(&String::from("default")),
                    ],
                )?
                .wait()?;
            }
            "close" => {
                excute(
                    self.config.clipath.clone(),
                    ["close", "--project", &self.rundir.to_string_lossy()],
                )?;
            }
            "upload" => {
                excute(
                    self.config.clipath.clone(),
                    [
                        "upload",
                        "--project",
                        &self.rundir.to_string_lossy(),
                        "-v",
                        task.params.get("-v").unwrap_or(&String::from("1.0.0")),
                        "-d",
                        task.params
                            .get("-m")
                            .unwrap_or(&String::from("\"common upload\"")),
                    ],
                )?
                .wait()?;
            }
            "new" => {
                let p = self
                    .temdir
                    .join(task.params.get("-temp").unwrap_or(&String::from("notemp")));
                let temp = Path::new(&p);
                copy_dir_all(temp, &self.rundir)?;
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
                println!("{}", self.rundir.display())
            }
            "help" => {
                println!(
                    "{}",
                    String::from_utf8_lossy(&fs::read(self.exedir.join("doc.md"))?)
                );
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
                        "templist" => {
                            let r = Path::new(&self.temdir).read_dir()?;
                            for item in r {
                                let item = item?;
                                if item.path().is_dir() {
                                    println!("{:?}", item.path().iter().last().unwrap());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            "direct" => {
                let mut aarg = Vec::new();
                for arg in args() {
                    aarg.push(arg);
                }
                aarg.remove(0);
                aarg.remove(0);
                println!("{:?}", aarg);
                excute(self.config.clipath.clone(), aarg)?;
            }
            _ => {
                println!("key in help to get document")
            }
        }
        Ok(())
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    // if dir doesn't exist create a dir
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // iterate the source dir file
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            // if the path is a dir make a recursion
            copy_dir_all(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            // if it's a file copy it to the dir
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
