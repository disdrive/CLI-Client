mod download;
mod login;
mod upload;
pub mod utility;

use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[command(
    author = "TackleDevs",
    version = "1.0.0",
    about = "disdrive cli client",
    long_about = "File i/o Service using Discord as storage. \nAll options are exclusive. Specifying multiple options will result in an error.\nFor more information, or valuable issues to report, please visit this link: https://github.com/disdrive"
)]
struct Args {
    #[arg(short = 'L', long = "login", help = "login to disdrive.", conflicts_with_all=&["file_path", "dl_key", "list", "logout", "dllist", "public", "private"])]
    login: bool,
    #[arg(short = 'O', long = "logout", help = "logout from disdrive.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "dllist", "public", "private"])]
    logout: bool,
    #[arg(short = 'u', long = "upload", help = "upload file to disdrive.", conflicts_with_all=&["login", "dl_key", "list", "logout", "dllist", "public", "private"])]
    file_path: Option<String>,
    #[arg(short = 'd', long = "download", help = "download file from disdrive.", conflicts_with_all=&["login", "file_path", "list", "logout", "dllist", "public", "private"])]
    dl_key: Option<String>,
    #[arg(short = 'p', long = "public", help = "make file public.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "logout", "dllist", "private"])]
    public: Option<String>,
    #[arg(short = 'P', long = "private", help = "make file private.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "logout", "dllist", "public"])]
    private: Option<String>,
    #[arg(short = 'l', long = "list", help = "list files in disdrive.", conflicts_with_all=&["login", "file_path", "dl_key", "logout", "dllist", "public", "private"])]
    list: bool,
    #[arg(short = 'D', long = "dl_from_list", help = "Downloads files from the list in an interactive manner.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "logout", "public", "private"])]
    dllist: bool,
}

//disdrive -P buntin --D oasdifaisdufasodjfoisa

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match utility::init() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }
    let token = utility::read_token_info()
        .expect(&format!(
            "Failed to read token info.\nCheck if {} file accessible",
            utility::get_config_path()
                .expect("Failed to get config path")
                .to_str()
                .unwrap()
        ))
        .token;
    if token.is_empty() {
        println!("token is empty");
    }
    match args {
        Args { login: true, .. } => {
            login::interactive_login().await;
            println!("token is saved");
        }
        Args { logout: true, .. } => {
            //remove
            match utility::remove_token_info() {
                Ok(_) => println!("token is removed"),
                Err(e) => println!("error: {}", e),
            }
        }
        Args {
            file_path: Some(file_path),
            ..
        } => {
            //upload
            let path = Path::new(&file_path);
            match utility::is_regular_file(path) {
                Ok(is_regular_file) => {
                    if !is_regular_file {
                        println!(
                            "{} is not available\nIt could potentially be a directory or a symbolic link.",
                            file_path
                        );
                        return;
                    }
                    //pass
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            }
            match upload::file_upload(path).await {
                Ok(key) => {
                    println!("{} is uploaded successfully", file_path);
                    println!("key: {}", key)
                }
                Err(e) => println!("upload error: {}", e),
            }
        }
        Args {
            dl_key: Some(dl_key),
            ..
        } => match download::dl_from_key(&dl_key).await {
            Ok(path) => println!("{} is downloaded successfully", path.to_str().unwrap()),
            Err(e) => println!("download error: {}", e),
        },
        Args {
            public: Some(_public),
            ..
        } => {
            println!("comming soon...");
        }
        Args {
            private: Some(_private),
            ..
        } => {
            println!("comming soon...");
        }
        Args { list: true, .. } => {
            println!("comming soon...");
        }
        Args { dllist: true, .. } => {
            println!("comming soon...");
        }
        _ => {
            //print help
            println!("");
        }
    }
}
