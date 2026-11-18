// This op can avoid some unnecessary matches
// See this code:

/***
let mut file = match File::open(&path) {
    Err(why) => {
        eprintln!("couldn't open {}: {}", filename, why);
        exit(1);
    }
    Ok(file) => file,
};
***/

/// Here, we:
// Call a function
// Checks if it worked
// If it worked, extract the value
// If it failed, end the program

// Rust offers the '?' operator to simplify that
// We could write:
/***
let mut file = File::open(&path)?;
***/

// Thats equivalent to:

/***
let mut file = match File::open(&path) {
    Ok(file) => file,
    Err(error) => return Err(error.into()),
};
***/

/// The '?' do two things:
// If it receives "Ok(value)," it extracts and returns "value."
// If it receives "Err" (error), it returns that error from the current function early
/// '?' does not end the program. It returns the error from the current function

// For example:

use std::fs::File;
use std::io;

fn open_file() -> Result<File, io::Error> {
    let file = File::open("file.txt")?;

    Ok(file)
}

// If the file opens, the function returns Ok(file)
// If not, the function returns Err(error) automatically

// Note that the function has:
/// -> Result<File, io::Error>
// This is necessary because the ? operator needs a context capable of catching the propagated error
// You cant just  use the '?' on your 'fn main()', you need to change it to:
/// fn main() -> Result<(), std::io::Error>
// Now it works.
