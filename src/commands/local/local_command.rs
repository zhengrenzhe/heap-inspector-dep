use std::path::PathBuf;

use warp::Filter;

use crate::utils::browser::open_url;
use crate::utils::webpage::host_webpage;

pub async fn local_command(file: &PathBuf) {
    let api = warp::path("api")
        .and(warp::path::param())
        .map(|name: String| format!("api, {}!", name));

    let index = warp::path::end().map(|| host_webpage("index.html"));
    let static_files = warp::path!(String).map(|path: String| host_webpage(&path));

    let routes = warp::get().and(api.or(static_files).or(index));

    open_url("http://localhost:3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
