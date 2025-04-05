use std::process::{Command, Stdio};


// copy text to sys clipboard 
pub fn copy_to_clipboard(text: &str) {
    let mut cmd = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to start wl-copy");

    {
        let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
        use std::io::Write;
        stdin.write_all(text.as_bytes()).expect("failed to write to wl-copy");
    }

    cmd.wait().expect("failed to run wl-copy");
}
