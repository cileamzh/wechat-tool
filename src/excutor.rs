use std::{
    env::{args, current_dir, current_exe},
    fs,
    path::{Path, PathBuf},
};

use crate::{copy_dir_all, excute, Config, Task};

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
                        "appid" => {}
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

            "add" => {
                for (k, v) in task.params {
                    match k.as_str() {
                        "temp" => {
                            let temp = PathBuf::from(v);
                            if temp.is_dir() {
                                copy_dir_all(&temp, &&self.temdir.to_path_buf())?;
                            } else {
                                println!("please use correct temp")
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
