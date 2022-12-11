use std::path::PathBuf;

pub async fn local_command(file: &PathBuf) {
    println!("{:?}", file)
}
