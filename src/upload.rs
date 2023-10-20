use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::utility;

pub async fn file_upload(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    let part = Part::stream(buffer).file_name(path.to_string_lossy().into_owned());
    let form = Form::new().part("file", part);
    let info = utility::read_token_info().expect("Failed to read token info");
    let client = Client::new();
    let res = client
        .post(format!("{}/file", info.server_url))
        .header(reqwest::header::AUTHORIZATION, info.token)
        .multipart(form)
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}
