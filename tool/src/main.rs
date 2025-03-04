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
            .required(false)
            .index(1))
        .arg(Arg::new("--explain")
            .long("--prev")
            .value_name("ex")
            .help("TBD"))
        .arg(Arg::new("--diff")
            .long("--diff")
            .value_name("dif")
            .help("TBD"))
        .subcommand(SubCommand::with_name("setup")
            .about("TBD"))
        .get_matches();
    
    if let Some(script) = matches.value_of("script") {
        doScript(script).await?;
    } else if let Some(ex) = matches.value_of("--explain") {
        println!("--explain place holder");
    } else if let Some(dif) = matches.value_of("--diff") {
        println!("--diff place holder");
    } else if let Some(matches) = matches.subcommand_matches("setup") {
        println!("setup place holder"); 

        // TODO add model once tuned


    } else {
        eprintln!("No valid script passed");
    }
    Ok(())
} // end main


/*
async fn doScript(script_path: &str, matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // start_ollama_server()?; // TODO once we know how

    let output = commands::runScript(script_path)?;

    match output {
        Ok((commands::OutputType::Stdout, out)) => {
            println!("{}", out);
            Ok(())  
        }

        Ok((commands::OutputType::Stderr, out)) => {
            let out = format!("Error was: {}", out);
            /*
            let ollama = Ollama::default();
            let mut stdout = stdout();
            let pb = setup_progress_bar(400);
            process_loop(&mut stdout, &ollama, &pb, false, &out, "Error output").await?;
            */

            // just print the error output for now
            println!("{}", out);
            Ok(())
        }

        Err(e) => {
            eprintln!("Error here was: {}", e);
            Ok(()) 
        }
    }  // close match
}  // end do
*/

async fn doScript(script_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //commands::start_ollama_server()?; // TODO LATER

    let out = commands::runScript(script_path);

    match out {
        Ok((commands::OutputType::Stdout, out))=> {
            println!("{}", out);
            Ok(())
        }

        Ok((commands::OutputType::Stderr, out)) => {
            let out = format!("{}", out);

            //let ollama = Ollama::default();
            // let mut stdout = stdout();
            // let pb = setup_progress_bar(400);
            // process_loop(&mut stdout, &ollama, &pb, false, &out, &postfix).await?;

            Ok(())
        }

        Err(e) => {
            eprintln!("Error was: {}", e);
            Ok(())
        }
    }
}