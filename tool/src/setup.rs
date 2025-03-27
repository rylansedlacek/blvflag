use std::fs;
use std::path::{Path};
use std::error::Error;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use ollama_rs::{Ollama, models::create::CreateModelRequest};
use std::env;
use std::process::{Command, Stdio};

const HF_TOKEN: &str = "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"; // the token

pub async fn setup_model() -> Result<(), Box<dyn Error>> {
    println!("Setting up model... \n");

    let model_url = "https://huggingface.co/rylansed/finetunedTest/resolve/main/model.safetensors"; 
    let model_name = "rylansed/finetunedTest";
    let model_dir = env::current_dir()?.join("models");

    if !model_dir.exists() {
        fs::create_dir_all(&model_dir)?;
    }

    let model_path = model_dir.join("model.safetensors"); // fix

    download_file(model_url, &model_path).await?;
 
    let ollama = Ollama::default();
    //
    let modelfile_path = model_dir.join("/Users/rylan/blvflag/tool/models/Modelfile"); // TODO fix

    /*
        The contents that it's writing here seems to be cause the overall issue. It says Cant find from or 
        file stuff which is literally in there not sure what the problem is
    */

    let modelfile_contents = format!( "from \"{}\"",model_path.display());

    fs::write(&modelfile_path, modelfile_contents)?;

    let response = ollama.create_model(CreateModelRequest::path("model".into(), modelfile_path.display().to_string().into())).await?;

    if response.message == "success" {
        println!("good run");
    } else {
        println!("bad run");
    }
    Ok(())
} // end setup

pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        println!("Oops!, the model has already been downloaded at {:?} \n", path);
        return Ok(());
    }

    println!("downloading model from {}", url);
    
    let client = Client::new();
    let mut response = client.get(url)
        .header("Authorization", format!("Bearer {}", HF_TOKEN))
        .send()
        .await?
        .bytes()
        .await?;
    
    let mut file = File::create(path).await?;
    file.write_all(&response).await?;

    println!("mdel downloaded successfully to {:?}", path);
    Ok(())
}
