use std::{fs, io, process::Command};

pub fn eula_agree(path: String, file_name: String) -> io::Result<()> {
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