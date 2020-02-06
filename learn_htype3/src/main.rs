//3、动态大小类型和Sized trait
//动态大小类型（dynamically sized types），有时被称为 “DST” 或 “unsized types”，
//这些类型允许我们处理只有在运行时才知道大小的类型。
//(1)最典型的就是str
fn main() {
    let s1: &str = "hello";
    let s2: &str = "world!";

    //let s1: str = "hello";
    //let s2: str = "world!";
    println!("Hello, world!");
}

//&str有两个值：str的地址和长度, &str的大小在编译时就可以知道，长度就是2*usize
//动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后，如：Box<str>\Rc<str>

//另一个动态大小类型是trait。每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型。
//为了将 trait 用于 trait 对象，必须将他们放入指针之后，
//比如 &Trait 或 Box<Trait>（Rc<Trait> 也可以）。




////（2） Sized trait
////为了处理 DST，Rust 用Sized trait 来决定一个类型的大小是否在编译时可知。
////这个 trait 自动为编译器在编译时就知道大小的类型实现。
////例子：
//
//fn generic<T>(t: T) {//T为编译时就知道大小的类型
//    // --snip--
//}
//
////等价于
//fn generic<T: Sized>(t: T) {//T为编译时就知道大小的类型
//    // --snip--
//}
//
////如何放宽这个限制呢？Rust提供如下方式：
//fn generic<T: ?Sized>(t: &T) {//T 可能是Sized，也可能不是 Sized 的
//    // --snip--
//}
