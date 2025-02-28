mod generate;
mod commands;

use tokio::{stdout}
use ollama_rs::Ollama;
use clap::{App, Arg, SubCommand}; 

// TODO main func



async fn logic(script_path &str) -> Result<(), Box<dyn std::error::Error>> {

    // start ollama server like blv run

    let out = commands::run_python_script(script_path);
    
    //decide what to do
    match out {
        Ok((commands::OutputType::Stdout, out)) => { // no issue
            println!("{}", out);
            Ok(())
        }

        // TODO add error handle Ok

        Err(e) => { // handle error
            eprintln!("Error: {}", e);
            Ok(())
        }
    }
}

