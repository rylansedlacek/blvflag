use std::fs::{self, File};  // 
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::io::copy;
use reqwest::blocking::Client; // use a blocking approach

/*
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, copy};
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
*/

/*
    For right now, this seems to work. Though I'm unsure about the async download_model
    stuff. I think that if we can ge the "worked" print to show up then I'd feel more 
    confident in the actual model being downloaded. Don't think it is because when I 
    run ollama list on cl it doesn't show as listed.

*/

const MODEL_URL: &str = "https://huggingface.co/rylansed/finetunedTest/"; // constant model url as string
const MODEL_PATH: &str = "models/finetunedTest.bin";  // path

pub fn setup_model() -> io::Result<()> {

    let model_dir = PathBuf::from("models");
    fs::create_dir_all(&model_dir)?; // get access to model folder on our build

    let model_path = model_dir.join("finetunedTest.bin"); // and path here

    if model_path.exists() {
        println!(""); // do nothing cause it exists
    } else {
        println!("downloading model... \n");
        download_model(MODEL_URL, &model_path);
        println!("model download completed! \n")
    } 

    //TOOD ADD AUTO START OF OLLAMA SERVER WHEN COMMAND RAN
    Command::new("ollama") // import it into ollama 
        .arg("import")
        .arg("finetunedTest")
        .arg("--model")
        .arg(model_path.display().to_string())
        .output()?;

    println!("started ollama");
    Ok(()) // ok out
}

async fn download_model(url: &str, path: &PathBuf) -> io::Result<()> { // actual download it off hugging face now

    let client = Client::new();
    let response = client.get(url).send().expect("Failed to send request"); 

    if !response.status().is_success() { // if not a success alert
        println!("failed to donwload the model");
    }

    let total_size = response.content_length().unwrap_or(0); // get the size

    let mut file = File::create(path)?; // create model file
    let mut downloaded: u64 = 0;

    let content = response.bytes().expect("Failed to get bytes");
    let mut content_ref = content.as_ref();
    copy(&mut content_ref, &mut file)?;

    println!("worked");
    Ok(())
}


/*
pub async fn download_model(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();
    let response = client.get(url).send().await?;

    let total_size = response.content_length().unwrap_or(0);
    let mut file = File::create(path);

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
       // pb.set_position(downloaded);
    }

    println!("model downloaded successfully");

    Ok(())
}
*/