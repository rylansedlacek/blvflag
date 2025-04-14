use std::fs;
use std::path::Path;
use similar::{TextDiff, ChangeTag};

pub fn compare_files(old: &Path, new: &Path) -> std::io::Result<String> {
    let old_contents = fs::read_to_string(old)?; // string-ify
    let new_contents = fs::read_to_string(new)?;
    let diff = TextDiff::from_lines(&old_contents, &new_contents);

    let mut output = String::new();

    // we iterate through each change,
    // - find delete append a - like git
    // - find an insert append a + like git

    for change in diff.iter_all_changes() { 
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
    Ok(output) // return
}
