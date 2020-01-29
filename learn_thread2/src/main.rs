use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("v: {:?}", v);
    });

    //println!("in main thread, v: {:?}", v);
    handle.join().unwrap();
    println!("Hello, world!");
}

//fn main() {
//    let v = vec![1, 2, 3];
//
//    let handle = thread::spawn(|| {
//        //sleep(10)
//        println!("v: {:?}", v);
//    });
//
//    //drop(v);
//
//    handle.join().unwrap();
//    println!("Hello, world!");
//}
