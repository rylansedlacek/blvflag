use std::fs;
use std::path::Path;
use similar::{TextDiff, ChangeTag};


/*
    This is a very basic version of diff that I believe will work for now for testing

*/
pub fn compare_files(old: &Path, new: &Path) -> std::io::Result<String> {
    let old_contents = fs::read_to_string(old)?; // string-ify
    let new_contents = fs::read_to_string(new)?;

    let diff = TextDiff::from_lines(&old_contents, &new_contents); // use this library found on google
    let mut output = String::new();

    for change in diff.iter_all_changes() { // iterate and add tags for changes literally same as docs
        match change.tag() {
            ChangeTag::Delete => {
                output.push_str(&format!("- {}", change));
            }
            ChangeTag::Insert => {
                output.push_str(&format!("+ {}", change));
            }
            ChangeTag::Equal => {}
        }
    }

    Ok(output)
}
