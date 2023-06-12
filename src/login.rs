use rpassword::prompt_password;
use std::io;
use std::io::Write;

pub fn interactive_login() {
    let mut user_id = String::new();
    print!("Please enter your user_id: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut user_id)
        .expect("Failed to read line");
    let user_id = user_id.trim();

    let password = prompt_password("Please enter your password:").expect("Failed to read password");
    println!("User_id: {}", user_id);
    println!("Password: {}", password);
}
