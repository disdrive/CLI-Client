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

pub fn is_regular_file(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    let is_dir = metadata.is_dir();
    let is_symlink = metadata.file_type().is_symlink();
    println!("is_dir():{}\nis_symlink():{}", is_dir, is_symlink);
    Ok(!(is_dir || is_symlink))
}
