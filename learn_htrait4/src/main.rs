//4、父 trait 用于在另一个 trait 中使用某 trait 的功能
//有时我们可能会需要某个 trait 使用另一个 trait 的功能。
//在这种情况下，需要能够依赖相关的 trait 也被实现。
//这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）。

use std::fmt;
trait OutPrint: fmt::Display { //要求实现Display trait
    fn out_print(&self) {
        let output = self.to_string();
        println!("output: {}", output);
    }
}

struct Point{
    x: i32,
    y: i32,
}

impl OutPrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    println!("Hello, world!");
}
