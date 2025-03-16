mod generate;
mod commands;
mod setup;

//use tokio::io::{stdout};
//use setup::setup_model;
use ollama_rs::Ollama;
use clap::{App, Arg, SubCommand};
use ollama_rs::generation::completion::request::GenerationRequest; 

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
        println!("--explain place holder"); // do we even need ask PC?
    } else if matches.is_present("diff") {
        println!("--diff place holder"); // do we even need ask PC?
    } else if matches.subcommand_matches("setup") {

        let model_file = matches.value_of("MODEL_FILE_LINK")
            .unwrap_or("the link")
            .to_string(); 

        let model_link = matches.value_of("MODEL_LINK")
            .unwrap_or("the link")
            .to_string();

        let dir = matches.value_of("DIRECTORY") //.map(PathBuf::from); //remove map stuff for now

        /* ASSUME CORRECT USAGE FOR NOW
        if (matches.value_of("MODEL_FILE_LINK").is_some() && matches.value_of("MODEL_LINK").is_none())
            || (matches.value_of("MODEL_LINK").is_some() && matches.value_of("MODEL_FILE_LINK").is_none()) {
            eprintln!("Both model file link and model link need to be specified if one is provided.");
            return Ok(());
        }
        */

        let file_urls = vec![ // store both in vec
            ("modelfile_path".to_string(), model_file), 
            ("model_path".to_string(), model_link) 
        ];

        setup_model(&file_urls, dir).await?; // pass this to the function

    } else {
        eprintln!("Error, invalid usage."); // add more verbose explanation
    } // end of matches

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
