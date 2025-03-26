use std::process::Command;

fn main() {
    
    let input_path = r"C:\Users\Administrator\Downloads\video.mp4";
    let output_path = r"C:\Users\Administrator\Downloads\compressed-video-x265.mp4";

    let output = Command::new("ffmpeg")
    .args(&["-y", "-i", input_path, 
    "-c:v", "libx265",  // video codec (h265)
    "-crf", "25",       // constant rate factor - video quality - the smaller the better quality (24-28 is a good balance)
    "-preset", "slow",  //encoding speed
    "-c:a", "aac",      // audio codec
    "-b:a", "128k",     // bitrate
    output_path])
    .output()
    .expect("failed to convert");

    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

}
