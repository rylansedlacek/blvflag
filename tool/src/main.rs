mod generate;
mod commands;

use tokio::io::{stdout};
use ollama_rs::Ollama;
use setup::setup_model;
use clap::{App, Arg, SubCommand};

//TODO FIX ALL OF THIS
// get base model make sure everything works
// host model on ollama
// ensure that reference model and print something using model output

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
        doScript(script, &matches).await?;
    } else if let Some(ex) = matches.value_of("explain") {
        println!("--explain place holder");
    } else if let Some(dif) = matches.value_of("diff") {
        println!("--diff place holder");
    } else if let Some(matches) = matches.subcommand_matches("setup") {
      
        let model_file = matches.value_of("MODEL_FILE_LINK") // do we need both? 
        .unwrap_or("TBD")
        .to_string(); 
        let model_link = matches.value_of("MODEL_LINK")
        .unwrap_or("TBD")
        .to_string();
         let directory = matches.value_of("DIRECTORY").map(PathBuf::from);

         // ADD CHECK if we need both

         let file_urls = vec![ // store in vector
            ("model_file".to_string(), model_file_link), 
            ("model_link".to_string(), model_link) 
        ];

        setup::setup_model(&file_urls, directory).await?; // set up the model using setup.rs

    } else {
        eprintln!("No valid command passed");
    }
    Ok(())
} // end main

async fn doScript(script_path: &str, matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    
    commands::start_ollama_server()?; // start the server
    let out = commands::runScript(script_path);

    match out {
        Ok((commands::OutputType::Stdout, out)) => {
            println!("{}", out);
            //Ok()
        }
        Ok((commands::OutputType::Stderr, out)) => {
            let out = format!("{}", out);
            let explain_this= matches.is_present("explain"); // check for flags
            let diff_code = matches.is_present("diff");

            if explain_this {
                println!("{}", script_path);
                
                //let ollama = Ollama::default(); // START OLLAMA HERE TO GET MODEL FOR EXPLAIN
                // let mut stdout = stdout();
                // let pb = setup_progress_bar(400);
                // process_loop(&mut stdout, &ollama, &pb, false, &out, &postfix).await?;
            }

            if diff_code {
                println!("{}", script_path);
                //TODO here we will have the diff stuff using autosave stuf TBD
            }
            //Ok()
        }
        Err(e) => {
            eprintln!("Error {}", e);
           // Ok()
        }
    }
    Ok(())
} //  end do script
