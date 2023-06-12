use rpassword::prompt_password;
use std::io;

pub fn interactive_login() {
    let mut user_id = String::new();
    loop {
        println!("Please enter a user_id (minimum 4 characters):");
        io::stdin()
            .read_line(&mut user_id)
            .expect("Failed to read line");
        if user_id.trim().len() >= 4 {
            break;
        }
        user_id.clear();
        println!("User_id must have at least 4 characters. Please try again.");
    }
    let password = prompt_password("Please enter your password:").expect("Failed to read password");
    println!("User_id: {}", user_id.trim());
    println!("Password: {}", password);
}
