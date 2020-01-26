//1、Drop trait类似于其它语言中的析构函数，当值离开作用域的时候执行此函数的代码。
//
struct Dog {
    name: String,
    //count: i32,
}

impl Drop for Dog{
    fn drop(&mut self) {
        println!("Dog {} leave", self.name);
        //self.count -= 1;
    }
}

fn main() {
    let a = Dog{name: String::from("wangcai")};
    { 
        let b = Dog{name: String::from("dahuang")};
        println!("0 +++++++++++++++++++");
    }

    println!("1 +++++++++++++++++++");

    println!("Hello, world!");
}
