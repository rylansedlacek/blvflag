use std::fs::{OpenOptions};
use std::io::{self, Write};
use std::fs;
use dirs;

/*
- I think that this is ok because we are making the user provide the key and storing it locally!
*/

pub async fn setup_model() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to blvflag setup.");
    println!("\nEnter your LLama API key:");

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();

    let mut key_dir = dirs::home_dir().expect("Failed to get home directory");
    key_dir.push("blvflag/tool/key");

    fs::create_dir_all(&key_dir)?; // create the directory to store our key
    let key_file_path = key_dir.join("api_key"); 

    let mut file = OpenOptions::new() // write the API key to the file
        .create(true)
        .write(true)
        .truncate(true) // and overwrite it if we need to
        .open(&key_file_path)?;
    writeln!(file, "{}", api_key)?;

    println!("Success! API key saved at {:?}", key_file_path); // notify
    Ok(())
}

