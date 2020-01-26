//1 rust通过所有权机制来管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
//
//2 堆和栈
//编译的时候数据的类型大小是固定的，就是分配在栈上的
//编译的时候数据类型大小不固定，就是分配堆上的
//
//3 作用域:{}
//
//4 String内存回收
//5 移动
//6 clone
//7 栈上数据拷贝
//8 函数和作用域
fn main() {
    let x: i32 = 1;

    {
        let y: i32 = 1;
        println!("x = {}", x);
        println!("y = {}", y);
    }

    {
        let s1 = String::from("hello");
        //s1.push_str(" world");
        println!("s1 = {}", s1); //String类型离开作用域的时候会调用drop方法

        let s2 = s1; 
        println!("s2= {}", s2);
        //println!("s1= {}", s1); //move to s2, s1 invalid
        //

        //clone
        let s3 = s2.clone();
        println!("s3= {}", s3);
        println!("s2= {}", s2);
    }

    //copy trait
    let a = 1;
    let b = a;
    println!("a = {}, b = {}", a, b);
    //常用的具有copy trait有：
    //所有的整型
    //浮点型
    //布尔值
    //字符类型 char
    //元组



    println!("Hello, world!");

}
