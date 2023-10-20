use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::utility;

pub async fn file_upload(path: &Path, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    let part = Part::stream(buffer).file_name(path.to_string_lossy().into_owned());
    let form = Form::new().part("file", part);
    let token = utility::read_token_info()
        .expect("Failed to read token info")
        .token;
    let client = Client::new();
    let res = client
        .post(format!("{}/file", url))
        .header(reqwest::header::AUTHORIZATION, token)
        .multipart(form)
        .send()
        .await?;
    println!("Response: {}", res.status());
    println!("{:?}", res);

    let body = res.text().await?;
    Ok(body)
}
