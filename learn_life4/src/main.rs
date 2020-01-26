//方法中的生命周期
struct StuA<'a> {
    name: &'a str,
}

impl<'b> StuA<'b> {
    fn do_something(&self) -> i32 {
        3
    }

    fn do_something2(&self, s: &str) -> &str{
    //fn do_something2<'b>(&'b self, s: &str) -> &'b str{
        self.name
    }

    fn do_something3<'a>(&self, s: &'a str) -> &'a str{
        s
    }
}

fn main() {
    let s = String::from("hello");
    let a = StuA{name: &s};
    println!("{}", a.do_something());
    let s2 = String::from("hello");
    println!("{}", a.do_something2(&s2));

    println!("{}", a.do_something3(&s2));
    println!("Hello, world!");
}
