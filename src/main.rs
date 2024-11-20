use std::{
    io::{stdin, Read, Write},
    process::{Command, Stdio},
};

fn copy_to_clipboard(clipboard: &str, value: &str) {
    let args = ["-selection", clipboard, "-i"];
    let process = Command::new("xclip")
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    process.stdin.unwrap().write_all(value.as_bytes()).unwrap();
}

fn copy_to_both_clips(value: &str) {
    copy_to_clipboard("primary", value);
    copy_to_clipboard("clipboard", value);
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    copy_to_both_clips(&buffer);
}
