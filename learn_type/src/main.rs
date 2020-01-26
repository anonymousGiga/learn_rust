fn main() {
    //bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);

    let is_false = false;
    println!("is_false = {}, {}", is_false, is_true);

    //char 在rust里面，char是32位的
    let a = 'a';
    println!("a = {}", a);

    let b = '你';
    println!("b = {}", b);
    
    //i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    let c: i8 = -111;
    println!("c = {}", c);

    let d: f32 = 0.0009;
    println!("d = {}", d);

    //自适应类型isize, usize
    println!("max = {}", usize::max_value());

    //数组
    //[Type; size] , size 也是数组类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let arr1: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    show(arr1);

    //元组
    let tup: (i32, f32, char) = (-3, 3.69, '好');
    let tup = (-3, 3.69, '好');
    println!("--------------------");
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("--------------------");
    
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("Hello, world!");
}

fn show(arr:[u32;3]) {
    println!("--------------------");
    for i in &arr {
        println!("{}", i);
    }
    println!("--------------------");
}
