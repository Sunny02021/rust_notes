// Create a function that takes an i32 and returns Some(number) if it is positive, and None otherwise.

fn is_positive(number: i32) -> Option<i32> {
    if number > 0 {
        Some(number)
    } else {
        None
    }
}

fn main() {
    let num = 10;
    let num1 = -10;
    let test = is_positive(num);
    let test1 = is_positive(num1);

    println!("{:?}", test);
    println!("{:?}", test1);
}
