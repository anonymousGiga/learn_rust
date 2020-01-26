//1、创建一个空String
//2、通过字面值创建一个String
//2.1、使用String::from()
//2.2、使用str的方式
//3、更新String
//3.1、push_str
//3.2、push
//3.3、使用“+”合并字符串
//3.4、使用format!
//4、String 索引
//5、str 索引
//6、遍历
//6.1、chars
//6.2、bytes
fn main() {
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);

    let s1 = String::from("init some thing");
    println!("{}", s1);

    let s1 = "init some thing".to_string();
    println!("{}", s1);

    let mut s2 = String::from("hello");
    s2.push_str(", world");
    let ss = " !".to_string();
    s2.push_str(&ss);
    println!("{}", s2);
    println!("ss = {}", ss);

    let mut s2 = String::from("tea");
    s2.push('m');
    //s2.push('mx'); //error
    //s2.push("x");  //error
    println!("{}", s2);

    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    //println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}", s341, s342, s343); //format!和println!类似
    println!("s344 = {}", s344);
    println!("s341 = {}", s341);
    println!("s342 = {}", s342);
    println!("s343 = {}", s343);

    let s4 = String::from("hello");
    //let s41 = s4[0];
    println!("s4.len = {}", s4.len());
    
    let s4 = String::from("你好");
    println!("s4.len = {}", s4.len());
    //let s41 = s4[0];

    let hello = "你好";
    let h5 = &hello[0..3];
    println!("h5 = {}", h5);

    //let h6 = &hello[0..2];
    //println!("h6 = {}", h6);

    //chars
    for c in s4.chars() {
        println!("c = {}", c);
    }

    println!("+++++++++++++++");
    //bytes
    for b in s4.bytes() {
        println!("b = {}", b);
    }
    println!("+++++++++++++++");
    println!("Hello, world!");
}
