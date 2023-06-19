use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    pub(crate) token: String,
}

fn get_config_path() -> io::Result<PathBuf> {
    let mut config_path = dirs::home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find home directory",
    ))?;
    config_path.push(".disdrive");
    Ok(config_path)
}

pub fn write_token_info(info: &TokenInfo) -> io::Result<()> {
    let config_path = get_config_path()?;
    let data = serde_json::to_string(info)?;
    fs::write(&config_path, data)?;
    Ok(())
}

pub fn read_token_info() -> io::Result<TokenInfo> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        write_token_info(&TokenInfo {
            token: String::new(),
        })?;
    }
    let data = fs::read_to_string(&config_path)?;
    let info: TokenInfo = serde_json::from_str(&data)?;
    Ok(info)
}

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
