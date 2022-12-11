use include_dir::{include_dir, Dir};
use warp::reply::WithHeader;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist/");

pub fn host_webpage<'a>(path: &str) -> WithHeader<&'a [u8]> {
    let bytes = match PROJECT_DIR.get_file(path) {
        Some(file) => file.contents(),
        None => &[],
    };

    return warp::reply::with_header(bytes, "content-type", match_mime(path));
}

fn match_mime(name: &str) -> &'static str {
    if name.ends_with(".html") {
        "text/html; charset=UTF-8"
    } else if name.ends_with(".css") {
        "text/css"
    } else if name.ends_with(".js") {
        "text/javascript"
    } else {
        "text/plain"
    }
}
