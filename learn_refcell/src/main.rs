//1、内部可变性：允许在使用不可变引用时改变数据。
//
//2、通过RefCell<T>在运行时检查借用规则（通常情况下，是在编译时检查借用规则），RefCell<T>代表其数据的唯一所有权。
//类似于Rc<T>，RefCell<T>只能用于单线程场景。
//
//3、选择Box<T>、Rc<T>或RefCell<T>的理由：
//Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
//Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
//因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));
    println!("a before: {:?}", a);
    println!("b before: {:?}", b);
    println!("c before: {:?}", c);

    println!("+++++++++++++++++++");
    *value.borrow_mut() += 10;
    println!("a after: {:?}", a);
    println!("b after: {:?}", b);
    println!("c after: {:?}", c);

    println!("Hello, world!");
}
