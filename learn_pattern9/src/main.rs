//7、匹配守卫提供额外的条件
//匹配守卫是一个指定于match分之模式之后的额外的if条件，必须满足才能选择此分支。
//fn main() {
//    let num = Some(4);
//    match num {
//        Some(x) if x < 5 => println!("< 5"),
//        Some(x) => println!("x: {}", x),
//        None => (),
//    };
//
//    println!("Hello, world!");
//}

fn main() {
    let num = Some(4);
    let y = 10; //位置1
    match num {
        Some(x) if x == y => println!("num == y"),//此处的y是位置1处的y
        Some(x) => println!("x: {}", x),
        None => (),
    };

    let x = 4;
    let y = false;
    match x {
        4|5|6 if y => println!("1"),//4|5|6 if y  a: 4|5|(6 if y)   b: ((4|5|6) if y)(等价于此种)
        _ => println!("2"),
    }
}
