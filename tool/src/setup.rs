use std::fs; // file stuff
use std::path::PathBuf; // file path
use std::process::Command; // need command.rs stuff
use reqwest::blocking::get; // think this works for HTTP

use std::io::{self, Write};
use tokio::fs as async_fs;

// complete guess and direct modification of blvrun
pub async fn setup_model(file_urls: &Vec<(String, String)>, directory: Option<PathBuf>) -> io::Result<()> {

    let base = directory.unwrap_or_else( || PathBuf::from("models"));

    fs::create_dir_all(&base_dir)?;

    for (filename, url) in file_urls {
        let path = base.join(filename);
        if !file_path.exists() {
            println!("File exists place holder")
            // TODO
        }
    } //  end for

    //  if we make it here the models are downloaded
    Command::new("ollama") // want to use ollama now
        .arg("import") // import based on docs
        .arg(base.join("modelfile_path").to_str())
        .output()?;
    
    println!("model imported right place holder")
    Ok(()) // ok out
 
} // end func
