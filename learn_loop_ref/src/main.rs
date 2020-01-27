#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("1, a rc count = {}", Rc::strong_count(&a));
    println!("1, a tail = {:?}", a.tail());

    {
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("2, a rc count = {}", Rc::strong_count(&a));
        println!("2, b rc count = {}", Rc::strong_count(&b));
        println!("2, b tail = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("3, a rc count = {}", Rc::strong_count(&a));
        println!("3, b rc count = {}", Rc::strong_count(&b));
        //println!("3, a tail = {:?}", a.tail()); //error
    }
}

