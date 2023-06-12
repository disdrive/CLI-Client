use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'L', long = "login", help = "login to disdrive.")]
    login: bool,
    #[arg(short = 'l', long = "list", help = "list files in disdrive.")]
    list: bool,
}

fn main() {
    let opts = Args::parse();

    match opts {
        Args { login: true, .. } => println!("login"),
        Args { list: true, .. } => println!("list"),
        _ => println!("nothing"),
    }
}
