use std::io::{self, Read};
use std::process::{Command, Stdio};

pub enum OutputType {
    Stdout,
    Stderr,
}

pub fn run_python_script(script_path: &str) -> io::Result<(OutputType, String)> {
    let output = Command::new("python3") // run
        .arg(script_path)
        .stdout(Stdio::piped()) // pipe
        //.stderr(Stdio::piped())
        .output()?;

    let out = if output.status.success() {
        (OutputType::Stdout,format!("{}", 
        String::from_utf8_lossy(&output.stdout)),)
    }; 
    
    /* TODO for ERROR 
    else {
        (OutputType::Stderr,format!("{}", 
        String::from_utf8_lossy(&output.stderr)),)
    };
    */

    Ok(out)
}


//TODO add starting of ollama server 