//解构并分解值
//解构元祖、结构体、枚举、引用
//
//解构结构体
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    //let p = Point{x: 1, y: 2};
    //let Point{x: a, y: b} = p;//变量a和变量b匹配x和y
    //assert_eq!(1, a);
    //assert_eq!(2, b);
    //
    //let Point{x, y} = p;
    //assert_eq!(1, x);
    //assert_eq!(2, y);

    let p = Point{x: 1, y: 0};
    match p {
        Point{x, y: 0} => println!("x axis"),
        Point{x: 0, y} => println!("y axis"),
        Point{x, y} => println!("other"),
    };

    println!("Hello, world!");
}
