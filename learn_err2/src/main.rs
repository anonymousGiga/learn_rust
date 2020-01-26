//1、当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外，还可以将错误传给调用者，让调用者决定如何处理，这被称为传播错误。
//2、传播错误的简写方式，提倡的方式
//3、更进一步的简写
//4、什么时候用panic！，什么时候用Result
//(1)示例、代码原型、测试用panic!\unwrap\expect
//(2)实际项目中应该用Result
//5、Option和Result

use std::io;
use std::io::Read;
use std::fs::File;
fn main() {
    println!("Hello, world!");
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("err = {:?}", e),
    }
}

//fn read_username_from_file() -> Result<String, io::Error> {
//    let f = File::open("hello.txt");
//    let mut f = match f {
//        Ok(file) => file,
//        Err(error) => return Err(error),
//    };
//
//    let mut s = String::new();
//    match f.read_to_string(&mut s) {
//        Ok(_) => Ok(s),
//        Err(error) => Err(error),
//    }
//}
//
//fn read_username_from_file() -> Result<String, io::Error> {
//    let mut f = File::open("hello.txt")?;
//
//    let mut s = String::new();
//    f.read_to_string(&mut s)?;
//    Ok(s)
//}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
