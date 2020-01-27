#[derive(Debug)]
enum List {
    //Cons(i32, RefCell<Rc<List>>),
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};
use std::rc::Weak;

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
    println!("1, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("1, a tail = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Weak::new())));
    if let Some(link) = b.tail() {
        *link.borrow_mut() = Rc::downgrade(&a);
    }
    println!("2, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("2, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("2, b tail = {:?}", b.tail());


    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::downgrade(&b);
    }
    println!("3, a strong count = {}, weak count = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("3, b strong count = {}, weak count = {}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("3, a tail = {:?}", a.tail());

    println!("Hello, world!");
}

//弱引用Weak<T>
//特点：
//（1）弱引用通过Rc::downgrade传递Rc实例的引用，调用Rc::downgrade会得到Weak<T>类型的智能指针，同时将weak_count加1（不是将strong_count加1）。
//（2）区别在于 weak_count 无需计数为 0 就能使 Rc 实例被清理。只要strong_count为0就可以了。
//（3）可以通过Rc::upgrade方法返回Option<Rc<T>>对象。
