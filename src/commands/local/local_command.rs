use crate::utils::browser::open_workbench;
use std::path::PathBuf;

pub async fn local_command(file: &PathBuf) {
    open_workbench();
    println!("{:?}", file)
}
