// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// I AM DONE

#[derive(Debug)]
enum Message {
    Move { x: i64, y: i64 },
    Quit,
    Echo(String),
    ChangeColor(i64, i64, i64)
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move{ x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit
    ];

    for message in &messages {
        message.call();
    }
}
