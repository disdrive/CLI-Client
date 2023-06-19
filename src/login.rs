use crate::utility::{write_token_info, TokenInfo};
use reqwest::Client;
use rpassword::prompt_password;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct ResponseBody {
    success: bool,
    token: Option<String>,
    message: Option<String>,
}

async fn call_api(user_id: &str, passwd: &str) -> Result<ResponseBody, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://localhost:3000";
    let body = serde_json::json!({
        "user_id": user_id,
        "password": passwd,
    });
    println!("{}", body.to_string());
    let response = client.post(url).json(&body).send().await?;
    let response_body: Value = response.json().await?;
    let parsed_body: ResponseBody = serde_json::from_value(response_body)?;

    Ok(parsed_body)
}

pub async fn interactive_login() {
    let mut user_id = String::new();
    print!("Please enter your user_id: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut user_id)
        .expect("Failed to read line");
    let password = prompt_password("Please enter your password:").expect("Failed to read password");
    match call_api(user_id.trim(), &password).await {
        Ok(body) => {
            if body.success {
                println!("Login successful!");
                let info = TokenInfo {
                    token: body.token.unwrap(),
                };
                write_token_info(&info).expect("Failed to write token info");
            } else {
                println!("Login failed: {}", body.message.unwrap());
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
