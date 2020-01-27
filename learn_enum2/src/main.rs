enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
}

//等同于
//struct QuitMessage; //类单元结构体
//struct MoveMessage {
//  x: i32,
//  y: i32,
//}
//struct WriteMessage(String) 
//struct Change(i32, i32, i32)

impl Message {
    fn prin(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            //_ => println!("Write")
            Message::Write(s) => println!("Write = {}", s)
        }
    }
}

fn main() {
    let quit = Message::Quit;
    quit.prin();


    let mo = Message::Move{x: 10, y: 20};
    mo.prin();

    let wri = Message::Write(String::from("Hello"));
    wri.prin();

    let change = Message::Change(1, 2, 3);
    change.prin();
    println!("Hello, world!");
}
