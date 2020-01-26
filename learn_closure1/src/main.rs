//1、闭包是可以保存进变量或者作为参数传递给其它函数的匿名函数。闭包和函数不同的是，闭包允许捕获调用者作用域中的值。
//2、闭包的使用方式
//3、使用带有泛型和Fn trait的闭包
fn main() {
    let use_closure = || {
        println!("This is a closure");
    };
    use_closure();
    println!("Hello, world!");

    //闭包定义会为每个参数和返回值类型推导一个具体的类型，但是不能推导两次
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| {x + 1};
    let add_one_v4 = |x| x+1;

    let a = add_one_v1(5);//use function
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    //不能推导两次的例子
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s = {}", s);

    //let n = example_closure(5);
    let n = example_closure(5.to_string());
    println!("n = {}", n);

    //捕捉环境中的变量
    let i = 1;
    let exe = |x| {x+i};
    let r = exe(5);
    println!("r = {}", r);
}

//语法格式
fn add_one_v1(x: u32) -> u32 {
    x + 1
}

