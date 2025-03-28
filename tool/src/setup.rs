use std::fs;
use std::path::{Path};
use std::error::Error;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use ollama_rs::{Ollama, models::create::CreateModelRequest};
use std::env;
use std::process::Command;


const HF_TOKEN: &str = "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"; // the token

pub async fn setup_model() -> Result<(), Box<dyn Error>> {
    println!("Setting up model... \n");

    let model_url = "https://huggingface.co/TheBloke/TinyLlama-1.1B-intermediate-step-1431k-3T-GGUF/resolve/main/tinyllama-1.1b-intermediate-step-1431k-3t.Q2_K.gguf";
    let model_name = "test_model";

    let model_dir = env::current_dir()?.join("model_download");

    if !model_dir.exists() {
        fs::create_dir_all(&model_dir)?;
    }

    let model_path = model_dir.join("tinyllama-1.1b-intermediate-step-1431k-3t.Q2_K.gguf");

    download_file(model_url, &model_path).await?;
 
    let ollama = Ollama::default();
    let modelfile_path = "/Users/rylan/blvflag/tool/model_download/Modelfile"; // TODO
    let modelfile_contents = format!( "FROM {}",model_path.display());
    fs::write(&modelfile_path, modelfile_contents)?;

    let output = Command::new("ollama")
        .arg("create")
        .arg("test") // todo change
        .arg("-f")
        .arg(modelfile_path)
        .output()?;

        // TODO ADD ERR CATCH FOR INVALID DOWNLAOD HERE
    
    Ok(())
} // end setup

pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        println!("Oops!, the model has already been downloaded at {:?} \n", path);
        return Ok(());
    }

    println!("Downloading model...");
    
    let client = Client::new();
    let mut response = client.get(url)
        .header("Authorization", format!("Bearer {}", HF_TOKEN))
        .send()
        .await?
        .bytes()
        .await?;
    
    let mut file = File::create(path).await?;
    file.write_all(&response).await?;

    println!("Model download successful. \n Downloaded to {:?}", path);
    Ok(())
}
