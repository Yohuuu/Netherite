use std::io;
mod start_setup;
mod download_required_files;
use crate::start_setup::start_setup;
mod eula_agree;
use eula_agree::eula_agree;

fn main() {
    // variables
    let username = std::env::var("USERNAME").unwrap();

    loop {
        println!("What do you want to do? 0) Create a new server. 1) Open an existing server.");
    
        let mut option_choice = String::new();
    
        io::stdin().read_line(&mut option_choice).expect("Failed to read!");
    
        let option_choice: i32 = match option_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // matches the user choice
        match option_choice {

            // paper
            0 => {
                start_setup(username);
                break;
            }

            // vanilla
            1 => {
                println!("placeholder");
                break;
            }
            // user entered something that is not listed above
            _ => {
                println!("Invalid input, try again!");
                continue;
            }
        }
    }
}