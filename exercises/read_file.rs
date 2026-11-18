// Add a read_file function that takes a file path as an argument and returns Result<String, std::io::Error>. Use ? inside it.

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn read_file(filename: String) -> Result<String, std::io::Error> {
    let path = Path::new(&filename);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args() .collect();
    if args.len() != 2 {
        println!("Usage: cargo run <filename>");
        return Ok(());
    }

    let filename = &args[1];

    let size = read_file(filename.to_string())?.len();

    println!("{}", size);
    
    Ok(())
}
