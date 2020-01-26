const MAX_POINTS: u32 = 100000;
fn main() {
    //1、变量定义
    //定义变量用let，如果变量没有用mut，那么是不可变的
    //let name: type 
    let a = 1;
    //let a:u32 = 1;
    println!("a = {}", a);

    let mut b: u32 = 1;
    println!("b = {}", b);

    b = 2;
    println!("b = {}", b);

    //2、隐藏
    let b: f32 = 1.1;
    println!("b = {}", b);

    //3、常量
    println!("MAX_POINTS = {}", MAX_POINTS);
    println!("Hello, world!");
}
