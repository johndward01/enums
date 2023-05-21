enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        Move {1, 1};
        ChangeColor(255, 0, 0);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}