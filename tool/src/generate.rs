use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use crate::commands;
use std::error::Error;

// Same function that used to be inside of main.
// want to update this to have a generate loop as in blv run

// We want to store the outputs in a json file to keep conversation history for further context
// This is so if a user keeps working through multiple errors we can keep track of whats been accomplished already

// basically I'm pretty sure blv does it to allow for multiple turn response

pub async fn process_script(script_path: &str, explain: bool, diff: bool) -> Result<(), Box<dyn Error>> {
    commands::start_ollama_server()?; // start server
    let out = commands::run_script(script_path);

    match out {
        Ok((commands::OutputType::Stdout, output)) => {
            println!("{}", output); // just print stdout normally
        }

        Ok((commands::OutputType::Stderr, error_output)) => {
            let ollama = Ollama::default(); // get the error
            let model_name = "gemma:2b"; // TODO change this TO CURRENT

            let prompt = format!("provide error line number. explain this error in 3-4 bullet points. 
            just provide the bullet points and line number. :\n{}", error_output); // to mock our goal output for fine-tuned model

            if explain { // USES THE DOWNLOADED MODEL (will be called blvflag_model) when done
                let request = GenerationRequest::new(model_name.to_string(), prompt);
                let response = ollama.generate(request).await?;
                println!("Explanation:\n{}", response.response); 
            } 

            if diff {
                println!("{}", script_path);
                //TODO here we will have the diff stuff
            }
        }

        Err(_) => {
            eprintln!("Failed to print script output");
        }
    } // end match

    Ok(()) // done
}
