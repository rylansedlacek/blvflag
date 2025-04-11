use crate::commands;
use crate::diff;
use indicatif::{ProgressBar, ProgressStyle};
use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationContext, GenerationResponseStream,},Ollama,};
use std::error::Error;
use tokio::io::{AsyncWriteExt, Stdout};
use tokio_stream::StreamExt;
use std::path::Path;


pub async fn process_script(script_path: &str, explain: bool, diff: bool) -> Result<(), Box<dyn Error>> {
    commands::start_ollama_server()?; // start the Ollama server
    let out = commands::run_script(script_path);

    match out {
            Ok((commands::OutputType::Stdout, output)) => { // basic output
                println!("{}", output);
            }

            Ok((commands::OutputType::Stderr, error_output)) => { // error
                let ollama = Ollama::default();
                let mut context: Option<GenerationContext> = None; // store conversation context
                let pb = setup_progress_bar(100); // and also set up progress tracking
                let mut stdout = tokio::io::stdout(); // output

                if explain { // explain flag
                    loop {
                        let prompt = format!(
                            "Provide the error line number and explain the error in 3-4 bullet points. \
                            Just provide the bullet points and line number:\n{}",
                            error_output
                        );
                        process_loop(&mut stdout, &ollama, &pb, false, &prompt, "", &mut context).await?;
                        break; // for now break, later prompt for input
                    }
                }

                if diff { // diff flag

                    let script_name = Path::new(script_path) // get the path and unwrap it just like in generate
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string();

                        let history_path = format!("/Users/rylan/blvflag/tool/history/{}", script_name);
                        let history_path = Path::new(&history_path);

                        if !history_path.exists() { // if it doestn exiist
                            std::fs::create_dir_all("/Users/rylan/blvflag/tool/history")?; // create our dir
                           // std::fs::copy(script_path, history_path)?;
                            std::fs::copy(script_path, history_path)?;
                            println!("worked and created");
                        } else {
                            let diff_output = diff::compare_files(history_path, Path::new(script_path))?; // grabs the versions for comapres
                            println!("\n===== changes =====");
                            println!("{}", diff_output);
                            println!("=================================\n");
    
                            std::fs::copy(script_path, history_path)?; //overwrites the change remove
                        }
                }
            } // end error block

        Err(_) => {
            eprintln!("Failed to execute the script");
        }

    } // match
    Ok(())
}

pub fn setup_progress_bar(max_tokens: u64) -> ProgressBar { // same as used in setup
    let pb = ProgressBar::new(max_tokens);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.red/red] {pos:>7}/{len:7} {msg}")
            .expect("Failed to set progress bar template")
            .progress_chars("#>-"),
    );
    pb // return the bar
}

pub async fn process_loop(stdout: &mut Stdout, ollama: &Ollama, pb: &ProgressBar, show_stream: bool, query: &str, postfix: &str, context: &mut Option<GenerationContext>,) -> Result<(), Box<dyn std::error::Error>> {
    commands::start_ollama_server()?; // always start server
    let mut final_output = Vec::new(); // store responses

    stdout.flush().await?; // overwrite past output

    let mut request = GenerationRequest::new("test".into(), query.to_string()); // create the request
    if let Some(ctx) = context.clone() { // additional context
        request = request.context(ctx);
    }
    let mut stream: GenerationResponseStream = ollama.generate_stream(request).await?; // begin streaming

    while let Some(Ok(res)) = stream.next().await { // loop through the response strem
        for ele in res {
            if show_stream {
                stdout.write_all(ele.response.as_bytes()).await?; // print
                stdout.flush().await?;
            } else {
                final_output.push(ele.response);
                pb.inc(1); // increment progress bar
            }

            if let Some(final_data) = ele.final_data {
                *context = Some(final_data.context); // TODO this is where we will store context, overwrite once convo is done
            }
        }
    } // end while

    let full_output = final_output.join(""); // TODO combine stored and current for final output
    let full_output = format!("{}{}", postfix, full_output); // append postfix, depricated from blv run

    if !show_stream {
        pb.finish_with_message("generation complete"); // what it says
        stdout.write_all(full_output.as_bytes()).await?;
        stdout.flush().await?;
    }

    Ok(())
} // end loop

