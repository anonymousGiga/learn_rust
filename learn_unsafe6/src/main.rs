//7、实现不安全的trait
//（1）当至少有一个方法中包含编译器不能验证的不变量时，该trait就是不安全的。
//（2）在trait之前增加unsafe声明其为不安全的，同事trait的实现也必须用unsafe标记。
unsafe trait Foo {
    fn foo(&self);
}

struct Bar();

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}

fn main() {
    let a = Bar();
    a.foo();
    println!("Hello, world!");
}
