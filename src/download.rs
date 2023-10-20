use crate::utility;
use reqwest::header::{AUTHORIZATION, CONTENT_DISPOSITION};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn dl_from_key(key: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let info = utility::read_token_info().expect("Failed to read token info");
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/file", info.server_url))
        .query(&[("userId", info.user_id)])
        .query(&[("key", key)])
        .header(AUTHORIZATION, info.token)
        .send()
        .await?;

    let filename_result: Result<String, &'static str> =
        if let Some(content_disposition) = response.headers().get(CONTENT_DISPOSITION) {
            if let Ok(disposition) = content_disposition.to_str() {
                let parts: Vec<&str> = disposition.split(';').collect();
                let mut found_filename: Option<String> = None;
                for part in parts {
                    if part.trim().starts_with("filename=") {
                        found_filename = part
                            .split('=')
                            .nth(1)
                            .map(|s| s.trim().trim_matches('\"').to_string());
                        break;
                    }
                }
                found_filename.ok_or("Failed to extract filename")
            } else {
                Err("Failed to convert Content-Disposition header to string")
            }
        } else {
            Err("Content-Disposition header is missing")
        };

    let filename = match filename_result {
        Ok(name) => name,
        Err(e) => {
            println!("Warning: {}", e);
            "output.bin".to_string()
        }
    };

    let path = PathBuf::from(filename);
    let body = response.bytes().await?;
    let mut file = File::create(&path).await?;
    file.write_all(&body).await?;

    println!("File saved successfully.");
    Ok(path)
}
