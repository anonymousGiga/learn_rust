use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val = {}", val); //调用send的时候，会发生move动作，所以此处不能再使用val
    });

    let re = rx.recv().unwrap();
    println!("got: {}", re);
    println!("Hello, world!");
}
