enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => println!("Quit"),
            Message::Write(s) => println!("{}", s),
            Message::Move { x, y } => {
                println!("moved x:{}, y:{}", x, y);
            }
            Message::ChangeColor(255, 0, 0) => println!("R:{} G:{} B:{}", 0, 1, 2),
            _ => println!("Not a valid Message "),
        }
    }
}

fn main() {
    let quit = Message::Quit;
    quit.call();
    let write = Message::Write(String::from("Hello, Rust!"));
    write.call();
    let _move = Message::Move { x: 1, y: 1 };
    _move.call();
    let change_color_blue = Message::ChangeColor(0, 0, 255);
    change_color_blue.call();
}
