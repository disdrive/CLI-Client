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
    long_about = "File i/o Service using Discord as storage. \nAll options are exclusive. Specifying multiple options will result in an error.\nFor more information,check here https://github.com/disdrive"
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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args {
        Args { login: true, .. } => {
            login::interactive_login().await;
            println!("login");
        }
        Args { logout: true, .. } => {
            //logout
            println!("logout");
            // implement the logout logic here
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
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            }
            match upload::file_upload(path).await {
                Ok(key) => println!("{} is uploaded successfully", key),
                Err(e) => println!("upload error: {}", e),
            }
        }
        Args {
            dl_key: Some(dl_key),
            ..
        } => {
            //download
            println!("download file {}", dl_key);
            download::dl_from_key(&dl_key).await;
        }
        Args {
            public: Some(public),
            ..
        } => {
            //make file public(keys)
            println!("make file {} public", public);
            utility::setPublic(&public).await;
        }
        Args {
            private: Some(private),
            ..
        } => {
            //make file private(keys)
            println!("make file {} private", private);
            utility::setPrivate(&private).await;
            // implement the make file private logic here
        }
        Args { list: true, .. } => {
            //list
            println!("list files");
            // implement the list files logic here
        }
        Args { dllist: true, .. } => {
            //download files from list
            println!("download files from list");
            // implement the download files from list logic here
        }
        _ => {
            // print help
            println!("");
        }
    }
}
