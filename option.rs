// Option determines whether a value can exist or not
// It has 2 possibilities:
/*
enum Option<T> {
    Some(T),
    None,
}
*/

// In practice:
/*
let number: Option<i32> = Some(42);
let another_number: Option<i32> = None;
*/
// Some(42): there's a value and its 42
// None: there's no value

// Option<i32> is different from i32.
// i32 always represents a integer. Option<i32>
// represents something that can have a integer or not

fn find_user(id: i32) -> Option<String> {
    if id == 1 {
        Some(String::from("Sunny"))
    } else {
        None
    }
}

fn main() {
    let user = find_user(1);

    println!("{:?}", user);
}
