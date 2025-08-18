use std::fs::{OpenOptions, self};
use std::io::Write;
use dirs;
use reqwest::header::{AUTHORIZATION, HeaderValue}; // add for API

//TODO should add a fall back!

pub async fn setup_model() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to BLVFLAG Setup");
    println!("Fetching API key...");

    // TODO ask PC is this hardcodedness is ok.
    let endpoint = "http://3.87.249.63:8080/api/meta-key"; 
    let auth_token = "full_as_a_tick"; 

    let client = reqwest::Client::new();
    // makes the request
    let res = client.get(endpoint).header(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", auth_token))?).send().await?; 

    if !res.status().is_success() { println!("Error, no API key retrieved!"); return Ok(()); }

    let json: serde_json::Value = res.json().await?; // turn to json
    let api_key = json["api_key"]
        .as_str()
        .ok_or("API key not found in response")?; // fall back

        // same as before here:
    let mut key_dir = dirs::home_dir().expect("Failed to get home directory"); // grab home dir
    key_dir.push("blvflag/tool/key");
    fs::create_dir_all(&key_dir)?;
    let key_file_path = key_dir.join("api_key");

    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&key_file_path)?;
    writeln!(file, "{}", api_key)?; // write it

    println!("Success! API key saved at {:?}", key_file_path);
    Ok(())
}
