//1、类型别名
type Kilometers = i32;

fn main() {
    let x: i32 = 5;    
    let y: Kilometers = 6;
    let r: i32 = x + y;
    println!("x + y = {}", r);
    println!("Hello, world!");
}

//类型别名的主要用途是减少重复。
////（1）考虑如下类型：
//Box<dyn Fn() + Send + 'static>
////如代码：
//let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
//
//fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
//    // --snip--
//}
//
//fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//    // --snip--
//}
//
////使用别名，代码：
//type Thunk = Box<dyn Fn() + Send + 'static>;
//let f: Thunk = Box::new(|| println!("hi"));
//fn takes_long_type(f: Thunk) {
//    // --snip--
//}
//fn returns_long_type() -> Thunk {
//    // --snip--
//}

////（2）考虑如下例子：
//use std::io::Error; //标准库中的std::io::Error结构体代表了所有可能的I/O错误
//use std::fmt;
//pub trait Write {
//    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//    fn flush(&mut self) -> Result<(), Error>;
//    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
//}
//
////加上如下类型别名声明：
//type Result<T> = std::result::Result<T, std::io::Error>;//result<T, E> 中 E 放入了 std::io::Error
//
////代码就可以变成：
//pub trait Write {
//    fn write(&mut self, buf: &[u8]) -> Result<usize>;
//    fn flush(&mut self) -> Result<()>;
//
//    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
//}
