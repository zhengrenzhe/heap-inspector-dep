use std::path::PathBuf;

use warp::Filter;

use crate::utils::browser::open_workbench;

pub async fn local_command(file: &PathBuf) {
    open_workbench();

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
