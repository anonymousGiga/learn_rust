//对任何实现了特定trait的类型有条件的实现trait
trait GetName {
    fn get_name(&self) -> &String;
}

trait PrintName {
    fn print_name(&self);
}

impl<T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("name = {}", self.get_name());
    }
}

struct Student {
    name: String,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

fn main() {
    let s = Student{name: String::from("xiaohuang")};
    s.print_name();
    println!("Hello, world!");
}
