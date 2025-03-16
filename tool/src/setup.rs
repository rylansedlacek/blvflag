/*
Eventually once a model is finetuned this will be used to get the model off of hugging face.
Additionally, changes will be made to allow users to run a "setup" command in order to
accomplish this and run this function. 
*/

/* IMPORTS:
use std::fs; // file stuff
use std::path::PathBuf; // file path
use std::process::Command; // need command.rs stuff
use reqwest::blocking::get; // think this works for HTTP
use std::io;
use tokio::fs as async_fs;
*/

/* SET UP MODEL FROM URL LINK
pub async fn setup_model(file_urls: &Vec<(String, String)>, dir: Option<PathBuf>) -> io::Result<()> {
    let base = dir.unwrap_or_else(|| PathBuf::from("models"));
    fs::create_dir_all(&base)?;

    for (filename, url) in file_urls { // going to loop through each file url
        let path = base.join(filename); // append
        if !path.exists() {
            println!("downloading {}...", filename); 
            let response = get(url)?; 
            let mut file = fs::File::create(&path)?; // get file path
            io::copy(&mut response.bytes()?.as_ref(), &mut file)?; //
        }
    }

    Command::new("ollama")
        .arg("create")
        .arg("blvflag-model-test")
        .arg("--model")
        .arg("gemma:2b")  // TODO eventually changes this to be the actual model we download
        //.arg("path")
        .output()?;

    println!("import successful.");
    Ok(())
}
*/
