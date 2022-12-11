pub fn open_url(url: &str) {
    match open::that(url) {
        Ok(_) => {}
        Err(err) => eprintln!("An error occurred when opening '{}': {}", url, err),
    }
}

pub fn open_workbench() {
    let url = if cfg!(debug_assertions) {
        "http://localhost:3000"
    } else {
        "https://xxx.xx"
    };
    open_url(url);
}
