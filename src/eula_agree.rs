use std::{fs, io, process::Command};

pub fn eula_agree(path: String, file_name: String) -> io::Result<()> {
    let mut agreed_to_eula = String::new();

    // doing that one more time because the server wont be fully set-up without doing this command 2 times
    Command::new("java")
        .current_dir(&path)
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
        let trimmed_agreed_to_eula = agreed_to_eula.trim();
     
        // matches the user's choice
        match trimmed_agreed_to_eula{
            "y" => {
                
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

        // trims the user option input
        let trimmed_launch_server = launch_server.trim();

        match trimmed_launch_server{
            "y" => {
            println!("Launching the server");
            Command::new("java")
                .current_dir(&path)
                .arg(format!("-jar"))
                .arg(format!("{}", &file_name))
                .output()
                .expect("Failed to launch the server");          
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