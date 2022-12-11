use std::path::PathBuf;

use include_dir::{include_dir, Dir};
use warp::Filter;

use crate::utils::browser::open_workbench;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist/");

pub fn host_webpage<'a>(path: String) -> &'a [u8] {
    match PROJECT_DIR.get_file(path) {
        Some(file) => file.contents(),
        None => &[],
    }
}

pub async fn local_command(file: &PathBuf) {
    open_workbench();

    let api = warp::path("api")
        .and(warp::path::param())
        .map(|name: String| format!("api, {}!", name));

    let index =
        warp::path::end().map(|| warp::reply::html(host_webpage(String::from("index.html"))));
    let static_files = warp::path!(String).map(|path| host_webpage(path));

    let routes = warp::get().and(api.or(static_files).or(index));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
