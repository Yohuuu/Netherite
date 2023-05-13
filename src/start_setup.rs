use std::{io::{self, Write}, fs::{self, File}, process::Command};
use crate::download_required_files::download_required_files;

pub fn start_setup(username: String){
    let core_choice = String::new();
    let mut url = String::new();
    let mut folder_name_choice = String::new();
        // loop in case the user enters something that is not an integer or not listed in available core choices
        loop {
            println!("What server software do you wish to use?\nCurrent options: 0) Paper, 1) Vanilla, 2) Forge");
        
            let mut core_choice = String::new();
        
            io::stdin().read_line(&mut core_choice).expect("Failed to read!");
        
            let core_choice: i32 = match core_choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        
            // matches the user choice
            match core_choice {
    
                // paper
                0 => {
                    url = "https://api.papermc.io/v2/projects/paper/versions/1.19.4/builds/519/downloads/paper-1.19.4-519.jar".to_string();
                    break;
                }
    
                // vanilla
                1 => {
                    url = "https://piston-data.mojang.com/v1/objects/8f3112a1049751cc472ec13e397eade5336ca7ae/server.jar".to_string();
                    break;
                }
    
                // forge
                2 => {
                    url = "https://maven.minecraftforge.net/net/minecraftforge/forge/1.19.4-45.0.49/forge-1.19.4-45.0.49-installer.jar".to_string();
                    break;
                }
    
                // user entered something that is not listed above
                _ => {
                    println!("Invalid input, try again!");
                    continue;
                }
            }
            }
    
        loop{
        println!("Enter a name for your server folder: ");
    
        io::stdin()
            .read_line(&mut folder_name_choice)
            .expect("Failed to get the name for the folder!");
    
        let mut trimmed_folder_name_choice = folder_name_choice.trim();
        // makes a directory with user's Windows username, and a folder name that they've chosen
        let mut path = format!("C:\\Users\\{}\\Desktop\\{}", username, trimmed_folder_name_choice);
    
        match fs::create_dir(&path){
            Ok(_) => {
                println!("Folder created!");
                download_required_files(&url, &path);
                break;
            }
            Err(e) => {
                println!("An error occured when creating the folder! {}", e);
                folder_name_choice = String::new();
                path = String::new();
                continue
            }
        };
        }
    }