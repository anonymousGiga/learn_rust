//4、解引用裸指针
//不可变和可变的，分别写作*const T, *mut T
//
//(1)允许忽略借用规则，可以同时拥有不可变和可变的指针，或者是多个指向相同位置的可变指针
//（2）不保证指向的内存是有效的
//（3）允许为空
//（4）不能实现任何自动清理的功能
//
fn main() {
    let mut num = 5;
    //创建不可变和可变的裸指针可以在安全的代码中，只是不能在不安全代码块之外解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let add = 0x12345usize;
    let _r = add as *const i32;

    println!("Hello, world!");
}
