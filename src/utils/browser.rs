pub fn open_url(url: &String) {
    match open::that(url) {
        Ok(_) => {}
        Err(err) => eprintln!("An error occurred when opening '{}': {}", url, err),
    }
}
