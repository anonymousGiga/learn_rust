use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
//知识点：
//1、发送者的send方法返回的是一个Result<T,E>,
//如果接收端已经被丢弃了，将没有发送值的目标，此时发送会返回错误。
//2、接受者的recv返回值也是一个Result类型，当通道发送端关闭时，返回一个错误值。
//3、接收端这里使用的recv方法，会阻塞到有一个消息到来。我们也可以使用try_recv()，不会阻塞，会立即返回。

//1、Rust中一个实现消息传递并发的主要工具是通道。通道由两部分组成，一个是发送端，一个是接收端，发送端用来发送消息，接收端用来接收消息。发送者或者接收者任一被丢弃时就可以认为通道被关闭了。
//
//2、通道介绍
//（1）通过mpsc::channel，创建通道，mpsc是多个生产者，单个消费者；
//（2）通过spmc::channel，创建通道，spmc是一个生产者，多个消费者；
//（3）创建通道后返回的是发送者和消费者，示例：
//let (tx, rx) = mpsc::channel();
//let (tx, rx) = spmc::channel();
