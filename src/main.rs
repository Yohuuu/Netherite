use std::fs;
use std::io;
use std::fs::File;
use reqwest;
use std::path::PathBuf;
use std::error::Error;
use std::process::Command;

fn main() {
    // variables
    let username = std::env::var("USERNAME").unwrap();
    let core_choice = String::new();
    let mut folder_name_choice = String::new();
    let mut url = String::new();

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

    println!("Enter a name for your server folder: ");

    io::stdin()
        .read_line(&mut folder_name_choice)
        .expect("Failed to get the name for he folder!");

    let folder_name_choice = folder_name_choice.trim().to_lowercase();

    // makes a directory with user's Windows username, and a folder name that they've chosen
    let path = format!("C:\\Users\\{}\\Desktop\\{}", username, folder_name_choice);    

    // checks if an error has occured making a folder
    match fs::create_dir(&path){
        Ok(_) => println!("Folder created!"),
        Err(e) => println!("An error occured when creating the folder! {}", e)
    };

    match download_required_files(&url, &path){
        Ok(success) => success,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
}

fn download_required_files(url: &String, download_folder: &String) -> Result<(), Box<dyn Error>> {
    let mut agreed_to_eula = String::new();
    // Create a GET request to download the file
    let mut response = reqwest::blocking::get(url)?;

    // Extract the filename from the URL
    let file_name = match url.rsplit_once("/") {
        Some((_, name)) => name,
        None => "file",
    };

    // Create the output file path
    let mut out_path = PathBuf::from(&download_folder);

    out_path.push(&file_name);

    // Create the output file and copy the contents of the response into it
    let mut out_file = File::create(&out_path)?;

    println!("Downloading {}", &file_name);
    
    io::copy(&mut response, &mut out_file)?;

    // creating a command that extracts the .jar file contents(java -jar doesnt extract files without jar xf)
    println!("Extracting the .jar file contents");
    let output = Command::new("jar")
        .current_dir(&download_folder)
        .arg("xf")
        .arg(format!("{}", &out_path.display()))
        .output()
        .expect("Failed to extract the contents of .jar file!");

    if url == "https://maven.minecraftforge.net/net/minecraftforge/forge/1.19.4-45.0.49/forge-1.19.4-45.0.49-installer.jar"{
        let output = Command::new("java")
            .arg("-jar")
            .arg("forge-1.19.4-installer.jar")
            .arg("-installServer")
            .output()
            .expect("Failed to setup the server!");
    }
    // creating a command that finishes the server setup
    println!("Finishing the server download");
    let output = Command::new("java")
        .current_dir(&download_folder)
        .arg(format!("-jar"))
        .arg(format!("{}", &file_name))
        .output()
        .expect("Failed to setup the server files!");

    loop{
    println!("Do you agree to Minecraft's EULA (https://aka.ms/MinecraftEULA)?\ny for yes, n for no");

    // reads the user response
    io::stdin()
        .read_line(&mut agreed_to_eula)
        .expect("Failed to convert the response!");
    
    // trims the user choice so it doesnt have whitespaces and stuff like that
    let mut trimmed_agreed_to_eula = agreed_to_eula.trim();

    // matches the user's choice
    match trimmed_agreed_to_eula{
        "y" => {
            eula_agree(download_folder.to_string(), file_name.to_string());
            break;
        }
        "n" => {
            println!("You didn't agree to eula! How do you expect to make a server then?");
            agreed_to_eula = String::new();
            continue
        }
        _ => {
            println!("Invalid input! Try again!");
            agreed_to_eula = String::new();
            continue
        }
    }
    }
    Ok(())
}
fn eula_agree(path: String, file_name: String) -> io::Result<()> {
    // Read the file content as a string
    let eula_file = format!("{}/eula.txt", path); // The file path
    let content = fs::read_to_string(&eula_file)?; // Use ? to get the String value

    // Replace only the first occurrence of "false" with "true"
    let agreed_eula = content.replace("false", "true");

    // Write the new content back to the file
    fs::write(&eula_file, agreed_eula)?; // Use ? to propagate any error

    loop{
        let mut launch_server = String::new();
        println!("Do you want to launch the server? y for yes, n for no:");
        io::stdin()
            .read_line(&mut launch_server)
            .expect("Failed to read the user input!");

        let trimmed_launch_server = launch_server.trim();

        match trimmed_launch_server{
            "y" => {
            println!("Launching the server");
            let output = Command::new("java")
                .current_dir(&path)
                .arg(format!("-jar"))
                .arg(format!("{}", &file_name))
                .output()
                .expect("Failed to setup the server files!");
                break Ok(());
            }
            "n" => {
                break Ok(())
            }
            _ => {
                println!("Invalid input! Try again!");
                launch_server = String::new();
                continue;
            }
        }

    }
}