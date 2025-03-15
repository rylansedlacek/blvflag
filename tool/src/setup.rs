//use std::fs; // file stuff
//use std::path::PathBuf; // file path
//use std::process::Command; // need command.rs stuff
//use reqwest::blocking::get; // think this works for HTTP
//use std::io;
//use tokio::fs as async_fs;


/*
Eventually once a model is finetuned this will be used to get the model off of hugging face.
Additionally, changes will be made to allow users to run a "setup" command in order to
accomplish this and run this function. 
*/

/*
pub async fn setup_model(file_urls: &Vec<(String, String)>, directory: Option<PathBuf>) -> io::Result<()> {
    let base = directory.unwrap_or_else(|| PathBuf::from("models"));
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
        .arg("gemma:2b")  // use the small test gemma
        .output()?;

    println!("import successful.");
    Ok(())
}
*/


// TODO remove once finetuned model uploaded to hugging face:

/*
pub fn basic_setup() -> io::Result<()> { // just set up the ollama server for testing
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
 */
