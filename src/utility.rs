use std::fs;
use std::io;
use std::path::Path;

pub async fn setPublic(key: &str) {
    //後で実装
    println!("setPublic");
}

pub async fn setPrivate(key: &str) {
    //後で実装
    println!("setPrivate");
}

pub fn is_dir_or_link(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    println!(
        "is_dir():{}\nis_symlink():{}",
        metadata.is_dir(),
        metadata.file_type().is_symlink()
    );
    Ok(!(metadata.is_dir() || metadata.file_type().is_symlink()))
}
