// Receive an Option<i32> and use match to print the number or indicate that there is no value.

fn print_num(num: Option<i32>) {
    match num {
        Some(num) => println!("{}", num),
        None => println!("There's no value"),
    }
}

fn main() {
    print_num(Some(67));
    print_num(None);
}