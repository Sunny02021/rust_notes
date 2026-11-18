// Result: when a operation can fail
/***
let file = File::open("file.txt");
***/

// Opening a file can go wrong or right.
//The file may not exist, or you don't have permission, etc

// To represent that, we use Result<T, E>:
/***
enum Result<T, E> {
    Ok(T),
    Err(E),
}
***/

// Two possibilities:
/// Ok: Operation worked and contains the result
/// Err: Operation failed and contains the error

/***
let mut file = match File::open(&path) {
    Err(why) => {
        eprintln!("couldn't open {}: {}", filename, why);
        exit(1);
    }
    Ok => file,
}
***/

// File::open returns Result<File, std::io::Error>
// In other words...
/// If it works, returns Ok(File)
/// If it fails, returns Err(std::io::Error)
// The match analyses what of these possibilities happened

// A simplified form:
/***
match result {
    Ok(value) => {
        println!("Worked");
    }

    Err(error) => {
        println!("Failed. Error: {}", error);
    }
}
***/
