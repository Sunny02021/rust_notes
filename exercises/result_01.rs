// Create a function that takes a String and attempts to convert it to an i32 using parse(). Return Result<i32, ParseIntError>.

use std::num::ParseIntError;

fn convert(s: String) -> Result<i32, ParseIntError> {
    let converted_string = match s.parse::<i32>() {
        Err(why) => {
            eprintln!("failed to convert {}: {}", s, why);
            return Err(why);
        }
        Ok(converted_string) => converted_string,
    };

    Ok(converted_string)
}

fn main() {
    let s = String::from("123456");
    let res = convert(s);
    println!("{:?}", res);
}
