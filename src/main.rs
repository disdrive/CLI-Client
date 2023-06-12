mod login;

use clap::Parser;
use tokio::runtime::Runtime;

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
    #[arg(short = 'u', long = "upload", help = "upload file to disdrive.", conflicts_with_all=&["login", "dl_key", "list", "logout", "dllist", "public", "private"])]
    file_path: Option<String>,
    #[arg(short = 'd', long = "download", help = "download file from disdrive.", conflicts_with_all=&["login", "file_path", "list", "logout", "dllist", "public", "private"])]
    dl_key: Option<String>,
    #[arg(short = 'l', long = "list", help = "list files in disdrive.", conflicts_with_all=&["login", "file_path", "dl_key", "logout", "dllist", "public", "private"])]
    list: bool,
    #[arg(short = 'O', long = "logout", help = "logout from disdrive.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "dllist", "public", "private"])]
    logout: bool,
    #[arg(short = 'D', long = "dl_from_list", help = "Downloads files from the list in an interactive manner.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "logout", "public", "private"])]
    dllist: bool,
    #[arg(short = 'p', long = "public", help = "make file public.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "logout", "dllist", "private"])]
    public: Option<String>,
    #[arg(short = 'P', long = "private", help = "make file private.", conflicts_with_all=&["login", "file_path", "dl_key", "list", "logout", "dllist", "public"])]
    private: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args {
        Args { login: true, .. } => {
            login::interactive_login().await;
            println!("login");
        }
        Args {
            file_path: Some(file_path),
            ..
        } => {
            //upload
            println!("upload file {}", file_path);
            // implement the upload file logic here
        }
        Args {
            dl_key: Some(dl_key),
            ..
        } => {
            //download
            println!("download file {}", dl_key);
            // implement the download file logic here
        }
        Args { list: true, .. } => {
            //list
            println!("list files");
            // implement the list files logic here
        }
        Args { logout: true, .. } => {
            //logout
            println!("logout");
            // implement the logout logic here
        }
        Args { dllist: true, .. } => {
            //download files from list
            println!("download files from list");
            // implement the download files from list logic here
        }
        Args {
            public: Some(public),
            ..
        } => {
            //make file public(keys)
            println!("make file {} public", public);
            // implement the make file public logic here
        }
        Args {
            private: Some(private),
            ..
        } => {
            //make file private(keys)
            println!("make file {} private", private);
            // implement the make file private logic here
        }
        _ => {
            // print help
            println!("");
        }
    }
}
