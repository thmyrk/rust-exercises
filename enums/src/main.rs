#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> () {
        dbg!(self);
    }
}

fn main() {
    let quit = Message::Quit;
    let moove = Message::Move { x: 1, y: 2 };
    let write = Message::Write(String::from("A message"));
    let change_color = Message::ChangeColor(100, 80, 255);

    quit.call();
    moove.call();
    write.call();
    change_color.call();
}
