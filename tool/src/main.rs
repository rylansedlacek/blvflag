mod generate;
mod commands;
mod setup;
mod diff;

use clap::{App, Arg, SubCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("blvflag")
        .arg(Arg::new("script")  
            .help("The script to run.")
            .required(false)
            .index(1))
        .arg(Arg::new("explain") 
            .long("explain")      
            .help("Utlized to explain error messages in more verbose manner.")
            .takes_value(false))  
        .arg(Arg::new("diff")
            .long("diff")
            .help("Utilized to compare code changes for debugging.")
            .takes_value(false))  
        .subcommand(SubCommand::with_name("setup")
            .help("Run this command to download model to user machine.")
            .about("Downloads model to machine."))
        .get_matches();
    
        if let Some(script) = matches.value_of("script") { 
            let explain = matches.is_present("explain"); // booleans
            let diff = matches.is_present("diff");
            generate::process_script(script, explain, diff).await?;
        } else if matches.subcommand_matches("setup").is_some() {
            println!("Starting Server... \n");
            commands::start_ollama_server()?; // start the server
            setup::setup_model().await?;
        } else {
            eprintln!("Invalid usage: blvflag (script.py) (--flag)");
        }
    Ok(())
} // end main

