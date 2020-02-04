////6、忽略模式中的值
//fn foo(_: i32, y: i32) {
//    println!("y = {}", y);
//}
//
////trait A {
////    fn bar(x: i32, y: i32);
////}
////
////struct B {}
////
////impl A for B {
////    fn bar(_: i32, y: i32) {
////        println!("y = {}", y);
////    }
////}

//fn main() {
//    foo(1, 2);
//
//    let numbers = (1, 2, 3, 4);
//    match numbers {
//        (one, _, three, _) => {
//            println!("one: {}, three: {}", one, three);
//        },
//    }
//    println!("Hello, world!");
//}


fn main() {
    let _x = 5;
    let _y = 5;

    //let s = Some(String::from("hello"));
    //if let Some(_c) = s {
    ////if let Some(c) = s {
    //    println!("found a string");
    //}
    ////println!("s: {:?}", s);
    
    let s = Some(String::from("hello"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s: {:?}", s);
}
