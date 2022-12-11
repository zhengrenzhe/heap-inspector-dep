use std::path::PathBuf;

use crate::utils::browser::open_workbench;

pub fn local_command(file: &PathBuf) {
    open_workbench();
}
