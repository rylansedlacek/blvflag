mod generate;
mod commands;
mod setup;

//use tokio::io::{stdout};
use ollama_rs::Ollama;
use clap::{App, Arg, SubCommand};
use ollama_rs::generation::completion::request::GenerationRequest; 
//use setup::setup_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("blvflag")
        .arg(Arg::new("script")  
            .help("The script to run")
            .required(false)
            .index(1))
        .arg(Arg::new("explain") 
            .long("explain")      
            .help("TBD")
            .takes_value(false))  
        .arg(Arg::new("diff")
            .long("diff")
            .help("TBD")
            .takes_value(false))  
        .subcommand(SubCommand::with_name("setup")
            .about("TBD"))
        .get_matches();
    
    if let Some(script) = matches.value_of("script") { // cant use -- because of .long
        do_script(script, &matches).await?;
    } else if matches.is_present("explain") {
        println!("--explain place holder");
    } else if matches.is_present("diff") {
        println!("--diff place holder");
    } else if matches.subcommand_matches("setup").is_some() {

        /*
        TODO add for getting model off web, which will be accomplished in setup.rs
        Plan to fine-tune and push model to hugging face, but model will stay
        small enough that the setup will actually download and run the model locally,
        using ollama server.
        */

    } else {
        eprintln!("Error, invalid usage.");
    }
    Ok(())
} // end main

async fn do_script(script_path: &str, matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {

    commands::start_ollama_server()?; // start server
    let out = commands::run_script(script_path);

    match out {

        Ok((commands::OutputType::Stdout, output)) => {
            println!("{}", output); // just print stdout normally
        }

        Ok((commands::OutputType::Stderr, error_output)) => {
            let ollama = Ollama::default(); // get the error
            let prompt = format!("provider error line number. explain this error in 3-4 bullet points. 
            just provide the bullet points and line number. :\n{}", error_output); // to mock our goal output for fine-tuned model

            if matches.is_present("explain") { // explain flag present
                let request = GenerationRequest::new("gemma:2b".to_string(), prompt.to_string());
                let response = ollama.generate(request).await?;
                println!("Explanation:\n{}", response.response); 
            } 

            if matches.is_present("diff") {
                println!("{}", script_path);
                //TODO here we will have the diff stuff
                // for now just print the script path
            }
        }
        Err(_) => {
            eprintln!("Failed to print script output");
        }
    }
    Ok(())
} // end doScript
