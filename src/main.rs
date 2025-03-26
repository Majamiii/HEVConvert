// use std::path::Path;
use std::process::Command;
// use std::ffi::OsStr;
// use std::path::PathBuf;

fn main() {
    

    let output = Command::new("cmd")
    .args(&["/C", "ffmpeg -i C:/Users/Administrator/Downloads/zana-slika.jpg C:/Users/Administrator/Downloads/zana-slika4.bmp"])
    .output()
    .expect("failed to convert");

    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

    return;
}
