//1、关联类型在trait定义中指定占位符类型
//关联类型是一个将类型占位符与trait相关联的方式。
//trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。
//如此可以定义一个使用多种类型的 trait。
//pub trait Iterator {
//    type Item;
//    fn next(&mut self) -> Option<Self::Item>;
//}

pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}

struct A {
    value: i32,
}

impl Iterator1<i32> for A {
    fn next(&mut self) -> Option<i32> {
        println!("in i32");
        if self.value > 3 {
            self.value += 1;
            Some(self.value)
        } else {
            None
        }
    }
}

impl Iterator1<String> for A {
    fn next(&mut self) -> Option<String> {
        println!("in string");
        if self.value > 3 {
            self.value += 1;
            Some(String::from("hello"))
        } else {
            None
        }
    }
}

fn main() {
    let mut a = A{value: 3};
    //a.next();
    <A as Iterator1<i32>>::next(&mut a);  //完全限定语法，带上了具体的类型
    <A as Iterator1<String>>::next(&mut a);
    println!("Hello, world!");
}
