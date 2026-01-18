#[derive(Debug)]
enum Message {
    Resize,
    Move,
    Echo,
    Quit,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
