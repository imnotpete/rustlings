// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


#[derive(Debug)]
enum Message<'a> {
    Move {x:i32, y:i32},
    Echo (&'a str),
    ChangeColor (i32, i32, i32),
    Quit
}

impl Message<'_> {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo("hello world"),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
