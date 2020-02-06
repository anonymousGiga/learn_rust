//2、从不返回的never type
//Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
//我们更倾向于称之为 never type。在函数不返回的时候充当返回值
//fn bar() -> ! {
//    loop{}
//}

//以下例子为《Rust程序设计语言》中第二章“猜猜看”游戏的例子
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //continue 的值是 !。
                                //当 Rust 要计算 guess 的类型时，它查看这两个分支。
                                //前者是 u32 值，而后者是 ! 值。
                                //因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


//panic!
//Option<T> 上的 unwrap 函数代码：
//impl<T> Option<T> {
//    pub fn unwrap(self) -> T {
//        match self {
//            Some(val) => val,
//            None => panic!("called `Option::unwrap()` on a `None` value"),
//        }
//    }
//}
