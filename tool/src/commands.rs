use std::io::{self};
use std::process::{Command, Stdio};

pub enum OutputType {
    Stdout,
    Stderr,
}

pub fn runScript(script_path: &str) -> io::Result<(OutputType, String)> { // to pipe the script given
    let output = Command::new("python3") // use python 3 
        .arg(script_path)
        .stdout(Stdio::piped()) // for stdout
        .stderr(Stdio::piped()) // for stderr
        .output()?;

    let out; 
    if output.status.success() {
        out = (OutputType::Stdout, String::from_utf8_lossy(&output.stdout).to_string()); // standard out
    } else {
        out = (OutputType::Stderr, String::from_utf8_lossy(&output.stderr).to_string()); // stand error out
    }
    Ok(out) // return out as string back to main for model processing.
} // end runScript



pub fn start_ollama_server() -> io::Result<()> { // to start the ollama server
    let _server = Command::new("ollama") 
        .arg("serve")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    std::thread::sleep(std::time::Duration::from_secs(3)); // add sleep to give it time to actually start
    Ok(())
}
