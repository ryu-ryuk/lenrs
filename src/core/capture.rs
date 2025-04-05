use std::process::Command;
use tempfile::NamedTempFile;

// function to use slurp to get the screen selection rectangle
pub fn get_selection_area() -> String {

    let output = Command::new("slurp")
        .output()
        .expect("failed to run slurp.");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// capturing the selected area using grim 
pub fn screenshot_area(coords: &str) -> (String, NamedTempFile) {
    let tmpfile = NamedTempFile::new().expect("failed to create temp file");
    let path = tmpfile.path().to_str().unwrap().to_string();

    Command::new("grim")
        .args(["-g", coords, &path])
        .output()
        .expect("failed to run grim");

    (path, tmpfile) // keep it alive
}
