struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("{} leave", self.name);
    }
}

//rust提供了std::mem::drop()
fn main() {
    let a = Dog{name: String::from("wangcai")};
    let b = Dog{name: String::from("dahuang")};
    //b.drop();
    drop(b);
    drop(a);

    println!("0 ++++++++++++++++++++++");
    println!("Hello, world!");
}
