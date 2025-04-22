use crate::commands;
use crate::diff;

use indicatif::{ProgressBar, ProgressStyle};
use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationContext, GenerationResponseStream,},Ollama,};
use std::error::Error;
use tokio::io::{AsyncWriteExt, Stdout};
use tokio_stream::StreamExt;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use dirs;

pub async fn process_script(script_path: &str, explain: bool, diff: bool) -> Result<(), Box<dyn Error>> {
    commands::start_ollama_server()?; // start the Ollama server
    let out = commands::run_script(script_path);

    /*
        TODO, gotta fix the should save stuff its driving me crazy
        just wont seem to work but thats ok stuff to work on. Probably need to rework
        the logic to just be better
    */

    match out {
            Ok((commands::OutputType::Stdout, output)) => { // STANDARD OUT
                
                if !diff {
                    println!("{}", output);
                }

                let mut should_save = true;
                let script_name = Path::new(script_path).file_name().unwrap().to_string_lossy().to_string();
        
                let date_stamp = Local::now().to_string();
                let mut history_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
                history_dir.push("blvflag/tool/history/std_history");
                fs::create_dir_all(&history_dir)?;
        
                let json_name = format!("{}_{}.json", script_name.trim_end_matches(".py"), date_stamp); // if its a .py save it in json with date
                let full_path = history_dir.join(&json_name);
                let current_script_content = fs::read_to_string(script_path)?;
        
                
                if diff { // diff flag
                    let prefix = script_name.trim_end_matches(".py");
                    let mut all_versions: Vec<PathBuf> = vec![];
                    
                    // get all the stuff from stdout history
                    let mut std_history_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
                    std_history_dir.push("blvflag/tool/history/std_history");
                    let std_versions = fs::read_dir(&std_history_dir)?
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let path = entry.path();
                            let filename = path.file_name()?.to_string_lossy();
                            if filename.starts_with(prefix) && path != full_path {
                                Some(path)
                            } else {
                                None
                            }
                        });
                    all_versions.extend(std_versions);
                    
                    // get all the stuff from the stderr history same diff
                    let mut err_history_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
                    err_history_dir.push("blvflag/tool/history/err_history");
                    let err_versions = fs::read_dir(&err_history_dir)?
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let path = entry.path();
                            let filename = path.file_name()?.to_string_lossy();
                            if filename.starts_with(prefix) && path != full_path {
                                Some(path)
                            } else {
                                None
                            }
                        });
                    all_versions.extend(err_versions);
                    
                    all_versions.sort();
                    
                    if let Some(last_version) = all_versions.last() {
                        let previous_content = fs::read_to_string(last_version)?;
                    
                        //if previous_content == current_script_content {
                        //    should_save = false;
                        //}

                        should_save = previous_content != current_script_content; // TODO!!
                    
                    let diff_output = diff::compare_files(last_version, Path::new(script_path))?;
                        if diff_output.is_empty() {
                            println!("No changes made.");
                        } else {
                            println!("------changes------");
                            println!("{}", diff_output);
                            println!("-------------------");
                        }
                    } else {
                        println!("No prior version found to diff against.");
                    }
                } // end diff
                
                if should_save { // this is for file tracking
                    fs::write(&full_path, &current_script_content)?; 
                    println!("\nSaved most recent version at: {:?}", full_path); 
                }
            } // end stdout

            Ok((commands::OutputType::Stderr, error_output)) => { // STANDARD ERROR
                if !diff && !explain {
                    println!("Error Caught! Use --explain OR --diff for help.\n");
                    println!("{}", error_output);
                }

                commands::start_ollama_server()?;
                let ollama = Ollama::default();
                let mut context: Option<GenerationContext> = None;
                let pb = setup_progress_bar(100); 
                let mut stdout = tokio::io::stdout(); 

                let mut should_save = true;
                let script_name = Path::new(script_path).file_name().unwrap().to_string_lossy().to_string();
        
                let date_stamp = Local::now().to_string();
                let mut history_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
                history_dir.push("blvflag/tool/history/err_history");
                fs::create_dir_all(&history_dir)?;
        
                let json_name = format!("{}_{}.json", script_name.trim_end_matches(".py"), date_stamp); // if its a .py save it in json with date
                let full_path = history_dir.join(&json_name);
                let current_script_content = fs::read_to_string(script_path)?;

                if diff { // diff flag
                    let prefix = script_name.trim_end_matches(".py");
                    let mut all_versions: Vec<PathBuf> = vec![];
                    
                // get all the stuff from stdout history
                    let mut std_history_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
                    std_history_dir.push("blvflag/tool/history/std_history");
                    let std_versions = fs::read_dir(&std_history_dir)?
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let path = entry.path();
                            let filename = path.file_name()?.to_string_lossy();
                            if filename.starts_with(prefix) && path != full_path {
                                Some(path)
                            } else {
                                None
                            }
                        });
                    all_versions.extend(std_versions);
                    
                   // get all the stuff from stderr history
                    let mut err_history_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
                    err_history_dir.push("blvflag/tool/history/err_history");
                    let err_versions = fs::read_dir(&err_history_dir)?
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let path = entry.path();
                            let filename = path.file_name()?.to_string_lossy();
                            if filename.starts_with(prefix) && path != full_path {
                                Some(path)
                            } else {
                                None
                            }
                        });
                    all_versions.extend(err_versions);
                    
                    all_versions.sort();
                    
                    if let Some(last_version) = all_versions.last() {
                        let previous_content = fs::read_to_string(last_version)?;
                    
                        //if previous_content == current_script_content {
                        //    should_save = false;
                        //}

                        should_save = previous_content != current_script_content; // TODO!!
                    
                        let diff_output = diff::compare_files(last_version, Path::new(script_path))?;
                        if diff_output.is_empty() {
                            println!("No changes made.");
                        } else {
                            println!("------changes------");
                            println!("{}", diff_output);
                            println!("-------------------");
                        }
                    } else {
                        println!("No prior version found to diff against.");
                    }
                } // end diff
        
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

                if should_save { // this is for file tracking
                    fs::write(&full_path, &current_script_content)?; 
                    println!("\n Saved most recent version at: {:?}", full_path); 
                }
            } // end standard error
        Err(_) => {
            eprintln!("Failed to execute the script");
        }
    } // match
    Ok(()) 
} // end processing script

pub fn setup_progress_bar(max_tokens: u64) -> ProgressBar { // same as used in setup from blvrun
    let pb = ProgressBar::new(max_tokens);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.green/white] {pos:>7}/{len:7} {msg}")
            .expect("Failed to set progress bar template")
            .progress_chars("#>-"),
    );
    pb // return the bar
} // end setup

pub async fn process_loop(stdout: &mut Stdout, ollama: &Ollama, pb: &ProgressBar, show_stream: bool, query: &str, postfix: &str, context: &mut Option<GenerationContext>,) -> Result<(), Box<dyn std::error::Error>> {
    commands::start_ollama_server()?; // always start server
    let mut final_output = Vec::new(); // store responses

    stdout.flush().await?; // overwrite past output

    let mut request = GenerationRequest::new("test4".into(), query.to_string()); // create the request
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

