use std::{
    env::{args, current_exe},
    path::PathBuf,
};

use wechat_tool::{get_config, Excutor, Task};

static SET: &str = "setting.config";

fn main() -> std::io::Result<()> {
    // let s_path = local.join(SET_DIR).join(SET);
    let config = get_config(
        current_exe().unwrap().parent().unwrap().to_path_buf(),
        PathBuf::from(SET),
    )?;
    let mut excutor = Excutor::new(config);

    let task = Task::from(args());
    excutor.exe_task(task)?;

    Ok(())
}
