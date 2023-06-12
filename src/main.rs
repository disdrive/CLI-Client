use clap::Parser;

#[derive(Parser)]
#[command(
    author = "TackleDevs",
    version = "1.0.0",
    about = "disdrive cli client",
    long_about = "File i/o Service using Discord as storage. \nAll options are exclusive. Specifying multiple options will result in an error.\nFor more information,check here https://github.com/disdrive"
)]
struct Args {
    #[arg(short = 'L', long = "login", help = "login to disdrive.", conflicts_with_all=&["file_name", "dl_key", "list", "logout", "dl_keys", "public", "private"])]
    login: bool,
    #[arg(short = 'u', long = "upload", help = "upload file to disdrive.", conflicts_with_all=&["login", "dl_key", "list", "logout", "dl_keys", "public", "private"])]
    file_path: Option<String>,
    #[arg(short = 'd', long = "download", help = "download file from disdrive.", conflicts_with_all=&["login", "file_name", "list", "logout", "dl_keys", "public", "private"])]
    dl_key: Option<String>,
    #[arg(short = 'l', long = "list", help = "list files in disdrive.", conflicts_with_all=&["login", "file_name", "dl_key", "logout", "dl_keys", "public", "private"])]
    list: bool,
    #[arg(short = 'O', long = "logout", help = "logout from disdrive.", conflicts_with_all=&["login", "file_name", "dl_key", "list", "dl_keys", "public", "private"])]
    logout: bool,
    #[arg(short = 'D', long = "dl_from_list", help = "Downloads files from the list in an interactive manner.", conflicts_with_all=&["login", "file_name", "dl_key", "list", "logout", "public", "private"])]
    dllist: bool,
    #[arg(short = 'p', long = "public", help = "make file public.", conflicts_with_all=&["login", "file_name", "dl_key", "list", "logout", "dl_keys", "private"])]
    public: Option<String>,
    #[arg(short = 'P', long = "private", help = "make file private.", conflicts_with_all=&["login", "file_name", "dl_key", "list", "logout", "dl_keys", "public"])]
    private: Option<String>,
}

fn main() {
    let args = Args::parse();

    // use match to handle each command separately
    match args {
        Args { login: true, .. } => {
            println!("login");
            // implement the login logic here
        }
        Args {
            file_path: Some(file_name),
            ..
        } => {
            println!("upload file {}", file_name);
            // implement the upload file logic here
        }
        Args {
            dl_key: Some(dl_key),
            ..
        } => {
            println!("download file {}", dl_key);
            // implement the download file logic here
        }
        Args { list: true, .. } => {
            println!("list files");
            // implement the list files logic here
        }
        Args { logout: true, .. } => {
            println!("logout");
            // implement the logout logic here
        }
        Args { dllist: true, .. } => {
            println!("download files from list");
            // implement the download files from list logic here
        }
        Args {
            public: Some(public),
            ..
        } => {
            println!("make file {} public", public);
            // implement the make file public logic here
        }
        Args {
            private: Some(private),
            ..
        } => {
            println!("make file {} private", private);
            // implement the make file private logic here
        }
        _ => {
            println!("No command selected");
        }
    }
}
