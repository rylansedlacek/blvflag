use std::fs; // file stuff
use std::path::PathBuf; // file path
use std::process::Command; // need command.rs stuff
use reqwest::blocking::get; // think this works for HTTP

use std::io::{self, Write};
use tokio::fs as async_fs;

// complete guess and direct modification of blvrun
pub async fn setup_model(file_urls: &Vec<(String, String)>, directory: Option<PathBuf>) -> io::Result<()> {
    
    let base = directory.unwrap_or_else(|| PathBuf::from("models"));

    fs::create_dir_all(&base)?;

    for (filename, url) in file_urls { // going to loop through each file url
        let path = base.join(filename); // append
        if !path.exists() {
            println!("Downloading {}...", filename)
            let response = reqwest::blocking::get(urls)?; // get the file
            let mut file = fs::File::create(&path)?; // get file path
            io::copy(&mut response.bytes()?.asref(), &mut file)?; //
        }
    }

    println!("download success, adding to ollama.");

    Command::new("ollama")
        .arg("create")
        .arg("blvflag-model-test")
        .arg("--model")
        .arg("gemma:2b")  // use the small test gemma
        .output()?;

    println!("import successful.");
    Ok(())
} // end func
