use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::env;
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header;
use dirs;

const HF_TOKEN: &str = "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"; // rylans token

pub async fn setup_model() -> Result<(), Box<dyn Error>> {
    println!("Setting up model... \n");

    //test model url:
    //let model_url = "https://huggingface.co/TheBloke/TinyLlama-1.1B-intermediate-step-1431k-3T-GGUF/resolve/main/tinyllama-1.1b-intermediate-step-1431k-3t.Q2_K.gguf";
    //let model_url = "https://huggingface.co/rylansed/blvflag_llama2.0-GGUF/resolve/main/blvflag_llama2.0.Q3_K_M.gguf";
    //let model_url = "https://huggingface.co/rylansed/blvflag_llama2.0-GGUF/resolve/main/blvflag_llama2.0.Q6_K.gguf";

    let model_url ="https://huggingface.co/rylansed/blvflag_llama3.0-GGUF/resolve/main/blvflag_llama3.0.Q5_K_M.gguf";
    let model_dir = env::current_dir()?.join("model_download");

    if !model_dir.exists() {
        fs::create_dir_all(&model_dir)?;
    }

    /* Names thus far:
    //ggml-model-Q2_K_v2.gguf
    //tinyllama-1.1b-intermediate-step-1431k-3t.Q2_K.gguf
    //llama-3.2-1b.Q2_K.gguf
    //blvflag_llama.Q3_K_M.gguf
    //blvflag_llama2.0.Q3_K_M.gguf
    */

    let model_path = model_dir.join("blvflag_llama2.0.Q3_K_M.gguf"); // name of model.gguf here
    download_file(model_url, &model_path).await?;
    
    let mut modelfile_path: PathBuf = dirs::home_dir().expect("Failed to get home directory"); // TODO NOT WORKING!
    modelfile_path.push("blvflag/tool/model_download/Modelfile");
 
    let modelfile_path = "/Users/rylan/blvflag/tool/model_download/Modelfile"; // TODO
    let modelfile_contents = format!("FROM {}", model_path.display());
    fs::write(&modelfile_path, modelfile_contents)?;

    let _output = Command::new("ollama")
        .arg("create")
        .arg("blv_tiny") // todo change
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
    
    // based from blv run complex so will explain
    let client = Client::new();
    let response = client.head(url).send().await?; 

    let total_size = response 
        .headers()
        .get(header::CONTENT_LENGTH) 
        .and_then(|val| val.to_str().ok()) // make content string
        .and_then(|s| s.parse::<u64>().ok()) // make it 64 bit int
        .unwrap_or(0); // if fails just go for 0

    let pb = ProgressBar::new(total_size); // styling
    pb.set_style(ProgressStyle::default_bar()
        .template("[{bar:40.orange/green}] {bytes}/{total_bytes} ({eta})")? // default from the lib
        .progress_chars(">="));

    let mut res = client.get(url).header("Authorization", format!("Bearer {}", HF_TOKEN)).send().await?; // sent a get request for hf url
    let mut file = File::create(path).await?; 
    
    let mut downloaded: u64 = 0; // track how much we've gotten
    let _buf = vec![0; 8192]; // create a buffer via rust docs

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk).await?; // write chunks
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded); // update 
    }
    
    println!("Model download successful! Path: {:?}\n", path); // confirm
    Ok(())
} // end dl
