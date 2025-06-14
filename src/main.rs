use clap::Parser;
use std::io::BufRead;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let file = std::fs::File::open(&args.path)
        .expect("Could not open the file");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.contains(&args.pattern) {
            println!("line: {:?}", line);
        }
    }
}