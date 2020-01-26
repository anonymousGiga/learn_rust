////实现Deref trait允许我们重载解引用运算符。
//let a: A = A::new();//前提：A类型必须实现Deref trait
//let b = &a;
//let c = *b;//解引用

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); //解引用

    let z = Box::new(x);
    assert_eq!(5, *z);
    println!("Hello, world!");
}
