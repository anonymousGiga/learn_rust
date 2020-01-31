//1、通道类似于单所有权的方式，值传递到通道后，发送者就无法再使用这个值；
//2、共享内存类似于多所有权，即多个线程可以同时访问相同的内存位置。

//互斥器：mutex
//1、任意时刻，只允许一个线程来访问某些数据;
//2、互斥器使用时，需要先获取到锁，使用后需要释放锁。
//Mutex<T>

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }//离开作用域时，自动释放

    println!("m = {:?}", m);
}

//Mutex<T>是一个智能指针，lock调用返回一个叫做MutexGuard的智能指针
//内部提供了drop方法，实现当MutexGuard离开作用域时自动释放锁。
