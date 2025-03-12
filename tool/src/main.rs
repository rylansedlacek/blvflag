mod generate;
mod commands;

 // same imports as blvrun for now
//use generate::{process_loop, setup_progress_bar};
 // TODO add ollama server start here
use clap::{App, Arg, SubCommand};

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
        println!("setup place holder"); 
        // TODO add getting the model directly from hugging face once trained
    } else {
        eprintln!("No valid script passed");
    }
    Ok(())
} // end main

async fn doScript(script_path: &str, matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    
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