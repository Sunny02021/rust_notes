// enum defines a type that can assume different variants
/***
enum Direction {
    North,
    South,
    East,
    West,
}
***/

// We can do:
/***
let direction = Direction::North;

match direction {
    Direction::North => println!("North"),
    Direction::South => println!("South"),
    Direction::East => println!("East"),
    Direction::West => println!("West"),
}
***/

// Enum can also hold data:
/***
enum Message {
    Text(String),
    Number(i32),
    Empty,
}

let message = Message::Text(String::from("Hello"));

match message {
    Message::Text(text) => println!("{}", text),
    Message::Number(number) => println!("{}", number),
    Message::Empty => println!("Empty message"),
}
***/
