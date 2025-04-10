use std::fs;
use std::path::Path;
use similar::{ChangeTag, TextDiff};

pub fn compare_files(old_path: &Path, new_path: &Path) -> Result<String, std::io::Error> { 
    let old_content = fs::read_to_string(old_path)?; // store the old a new path
    let new_content = fs::read_to_string(new_path)?;

    let diff = TextDiff::from_lines(&old_content, &new_content); // use this library that I found

    let mut result = String::new();

    for change in diff.iter_all_changes() { // iterate through the chages
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };
        result.push_str(&format!("{}{}", sign, change)); // push the changes with signs to output
    }

    Ok(result) // and return result
}
