use std::{io, process::Command, path::PathBuf, fs::File, error::Error};
use crate::eula_agree;

pub fn download_required_files(url: &String, download_folder: &String) -> Result<(), Box<dyn Error>> {
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