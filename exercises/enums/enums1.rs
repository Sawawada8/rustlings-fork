// enums1.rs
// No hints this time! ;)

// // I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
    println!("first: {}, second: {}",value_in_cents(Message::Echo), value_in_cents(Message::ChangeColor));
}

/// match で 定数返す みたいな
fn value_in_cents(coin: Message) -> u32 {
    match coin {
        Message::Quit => 1,
        Message::Echo => 5,
        Message::Move => 10,
        Message::ChangeColor => 25,
    }
}
