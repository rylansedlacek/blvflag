use std::io::{self};
use std::process::{Command, Stdio};

pub enum OutputType {
    Stdout,
    Stderr,
}

pub fn run_script(script_path: &str) -> io::Result<(OutputType, String)> { // to pipe the script given

    let output = Command::new("python3") 
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
