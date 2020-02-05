//5、newtype 模式用以在外部类型上实现外部 trait
//孤儿规则（orphan rule）：只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。
//一个绕开这个限制的方法是使用 newtype 模式（newtype pattern）。

use std::fmt;
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0.join(","))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("World")]);
    println!("w = {}", w);
    println!("Hello, world!");
}
//说明：
//在上述例子中，我们在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外。我们可以创建一个包含 Vec<T> 实例的 Wrapper 结构体，然后再实现。
