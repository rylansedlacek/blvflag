use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::fs;


pub enum OutputType {
    Stdout,
    Stderr,
}

pub fn run_script(script_path: &str) -> io::Result<(OutputType, String)> { 
    let output = Command::new("python3") 
        .arg(script_path)
        .stdout(Stdio::piped()) 
        .stderr(Stdio::piped()) 
        .output()?;

    let out; 
    if output.status.success() {
        out = (OutputType::Stdout, String::from_utf8_lossy(&output.stdout).to_string());
    } else {
        out = (OutputType::Stderr, String::from_utf8_lossy(&output.stderr).to_string()); 
    }
    Ok(out)
} // end runScript

pub fn clear_history() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs_next::home_dir().ok_or("Unable to get home directory")?; // first get the users home dir
   
    let dirs_to_clear = vec![
        home_dir.join("blvflag/tool/history/std_history"), 
        home_dir.join("blvflag/tool/history/err_history"),
    ];

     print!("Confirm action (Y/n): ");
     io::stdout().flush()?; 

     let mut input = String::new();
     io::stdin().read_line(&mut input)?;
     let input = input.trim().to_lowercase();

     if input != "y" {
        println!("Aborted action.");
        return Ok(());
     }

    for dir in dirs_to_clear { 
        if dir.exists() {
            for entry in fs::read_dir(&dir)? {
                let path = entry?.path();
                if let Some(file_name) = path.file_name() {
                    if file_name != "placeholder.json" {
                        fs::remove_file(path)?;
                    }
                }
            }
            println!("Cleared all files in {:?}", dir); // notify
        } else {
            println!("{:?} does not exist.", dir);
        }
    }
    Ok(())
} // end clearHistory