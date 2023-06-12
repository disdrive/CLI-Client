use clap::Parser;

#[derive(Parser, Debug)]
pub struct Login {
    #[arg(short, long)]
    pub user_id: String,
    #[arg(short, long)]
    pub password: String,
}

#[derive(Parser, Debug)]
pub struct Upload {
    #[arg(short, long)]
    pub filename: String,
    #[arg(short = 'p', long)]
    pub public: bool,
    #[arg(short = 'P', long)]
    pub private: bool,
}

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Your Name")]
pub enum Opts {
    Login(Login),
    Upload(Upload),
    // ...
}
