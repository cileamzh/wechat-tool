use std::{
    collections::HashMap,
    env::Args,
    ffi::OsStr,
    fs::{self},
    io::{self},
    path::Path,
    process::{Child, Command},
};

pub mod config;
pub use config::{get_config, Config};

pub mod excutor;
pub use excutor::Excutor;

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
