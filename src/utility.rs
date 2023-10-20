use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub(crate) token: String,
    pub(crate) user_id: String,
    pub(crate) server_url: String,
}

#[derive(Deserialize)]
pub struct VisualityApiResponse {
    success: bool,
    message: String,
}

pub fn init() -> io::Result<()> {
    let config_path = get_config_path().expect("Failed to get config path");
    match config_path.exists() {
        true => match fs::read_to_string(&config_path) {
            Ok(data) => match serde_json::from_str::<ClientInfo>(&data) {
                Ok(_) => {
                    //do nothing
                    Ok(())
                }
                Err(_) => {
                    println!("{} is corrupted", config_path.to_str().unwrap());
                    let info = ClientInfo {
                        token: String::new(),
                        user_id: String::new(),
                        server_url: String::new(),
                    };
                    write_token_info(&info)
                }
            },
            Err(_) => {
                let info = ClientInfo {
                    token: String::new(),
                    user_id: String::new(),
                    server_url: String::new(),
                };
                write_token_info(&info)
            }
        },
        false => {
            let info = ClientInfo {
                token: String::new(),
                user_id: String::new(),
                server_url: String::new(),
            };
            write_token_info(&info)
        }
    }
}

pub fn get_config_path() -> io::Result<PathBuf> {
    let mut config_path = dirs::home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Could not find home directory",
    ))?;
    config_path.push(".disdrive");
    Ok(config_path)
}

pub fn write_token_info(info: &ClientInfo) -> io::Result<()> {
    let config_path = get_config_path()?;
    let data = serde_json::to_string(info)?;
    fs::write(&config_path, data)?;
    Ok(())
}

pub fn read_token_info() -> io::Result<ClientInfo> {
    let config_path = get_config_path()?;
    let data = fs::read_to_string(&config_path)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Failed to read token info"))?;

    serde_json::from_str(&data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse token info"))
}

pub fn remove_token_info() -> io::Result<()> {
    let config_path = get_config_path()?;
    fs::remove_file(&config_path)?;
    Ok(())
}

pub fn auth_str(token: &str) -> String {
    format!("Bearer {}", token)
}

pub async fn set_public(key: &str, url: &str, token: &str) -> Result<(), Error> {
    let client = Client::new();
    let res = client
        .post(format!("{}/api/set_public", url))
        .body(key.to_string())
        .header("authorization", auth_str(token))
        .send()
        .await?;
    let api_res: VisualityApiResponse = res.json().await?;

    if api_res.success {
        println!("success");
    } else {
        println!("error: {}", api_res.message);
    }
    Ok(())
}

pub async fn set_private(key: &str, url: &str, token: &str) -> Result<(), Error> {
    let client = Client::new();
    let res = client
        .post(format!("{}/api/set_private", url))
        .body(key.to_string())
        .header("authorization", auth_str(token))
        .send()
        .await?;
    let api_res: VisualityApiResponse = res.json().await?;
    if api_res.success {
        println!("success");
    } else {
        println!("error: {}", api_res.message);
    }
    Ok(())
}

pub fn is_regular_file(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    let is_dir = metadata.is_dir();
    let is_symlink = metadata.file_type().is_symlink();
    println!("is_dir():{}\nis_symlink():{}", is_dir, is_symlink);
    Ok(!(is_dir || is_symlink))
}
