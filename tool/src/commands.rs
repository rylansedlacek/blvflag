use std::io::{self};
use std::process::{Command, Stdio};

pub enum OutputType {
    Stdout,
    Stderr,
}

pub fn runScript(script_path: &str) -> io::Result<(OutputType, String)> {
    let output = Command::new("python3") // use python 3 
        .arg(script_path)
        .stdout(Stdio::piped()) // for stduout
        .stderr(Stdio::piped()) // for stderr
        .output()?;

    let out; // null decl
    if output.status.success() {
        out = (OutputType::Stdout, String::from_utf8_lossy(&output.stdout).to_string());
    } else {
        out  = (OutputType::Stderr, String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(out) // return the output converted to string format

} // end run script

pub fn start_ollama_server() -> io::Result<()> { // this code from blvrun should work fine
    Command::new("ollama")
        .arg("serve")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(())
}