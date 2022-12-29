use std::fs;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// the input your file logs here
   #[arg(short, long)]
   input_file: String,

   /// the output of your sorted file logs
   #[arg(short, long)]
   output_file: String,
}


fn main() {
    let args = Args::parse();

    let input_file = &args.input_file;
    let output_file = &args.output_file;

    let contents = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    let mut child = Command::new("sort")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Error starting 'sort' command");
   
    if let Err(e) = child.stdin.as_mut().unwrap().write_all(contents.as_bytes()) {
        println!("Error writing to 'sort' command: {}", e);
        std::process::exit(1);
    }

    let output = match child.wait_with_output() {
        Ok(output) => output,
        Err(e) => {
            println!("Error waiting for 'sort' command: {}", e);
            std::process::exit(1);
        }
    };

    if !output.status.success() {
        println!("'sort' command failed with status {}", output.status);
        std::process::exit(1);
    }

    match fs::write(output_file, output.stdout) {
        Ok(_) => println!("Sorted contents written to {}", output_file),
        Err(e) => println!("Error writing to output file: {}", e),
    }
}