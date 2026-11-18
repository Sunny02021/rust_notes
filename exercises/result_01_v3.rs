// Create a function that takes a String and attempts to convert it to an i32 using parse(). Return Result<i32, ParseIntError>.

use std::num::ParseIntError;

fn convert(s: String) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn main() {
    let s = String::from("123456");
    let res = convert(s);
    println!("{:?}", res);
}
