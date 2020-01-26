fn other_fun() {
    println!("This is a function");
}

fn other_fun1(a: i32, b: u32) {
//fn other_fun1(a, b) { //error
    println!("a = {}, b = {}", a, b);
}

fn other_fun2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

fn other_fun3(a: i32, b: i32) -> i32 {
    //let result = a + b;
    //result
    a + b
}

fn main() {
    other_fun();

    let a: i32 = -1;
    let b: u32 = 2;
    other_fun1(a, b);

    let c: i32 = 9;
    let r: i32 = other_fun2(a, c);
    println!("r = {}", r);

    let r2: i32 = other_fun3(a, c);
    println!("r2 = {}", r2);

    //语句是执行一些操作，但是不返回值的指令
    //let y = 1; //语句，不返回值
    //let x = (let y = 1);
    
    //表达式会计算一些值
    let y = {
        let x = 1;
        //x + 1;
        x + 1
    };

    println!("y = {}", y);

    println!("Hello, world!");
}
