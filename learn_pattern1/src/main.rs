//1、模式有两种：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称为是不可反驳的。对值进行匹配可能会失败的模式被称为可反驳的。
//
//2、只能接受不可反驳模式的有：函数、let语句、for循环。原因：因为通过不匹配的值程序无法进行有意义的工作。
//
//3、if let和while let表达式被限制为只能接受可反驳的模式，因为它们的定义就是为了处理有可能失败的条件。
//
fn main() {
    //let a: Option<i32> = Some(5); //匹配Some(value), None
    //let b: Option<i32> = None; //匹配Some(value), None
    ////let Some(x) = a;

    //if let Some(v) = a {
    if let v = 5 {
        println!("v {}", v);
    }

    println!("Hello, world!");
}
