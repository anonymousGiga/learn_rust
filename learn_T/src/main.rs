//1、泛型是具体类型或者其它属性的抽象替代，用于减少代码重复.
//2、在函数定义中使用泛型。
//3、在结构体中使用泛型。
//4、枚举中的泛型。
//5、方法中的泛型。
//6、总结：使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
////--------没有泛型-----------
////for i32
//fn largest_i32(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}
//
////for char
//fn largest_char(list: &[char]) -> char {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

////----------使用泛型-------------
//fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
//    let mut larger = list[0];
//    for &item in list.iter() {
//        if item > larger {
//            larger = item;
//        }
//    }
//    larger
//}
//
//fn main() {
//    let number_list = vec![1, 2, 23, 34, 8, 100];
//    //let max_number = largest_i32(&number_list);
//    let max_number = largest(&number_list);
//    println!("max_number = {}", max_number);
//
//    let char_list= vec!['a', 'y', 'b'];
//    //let max_char = largest_char(&char_list);
//    let max_char = largest(&char_list);
//    println!("max_char = {}", max_char);
//    println!("Hello, world!");
//}


////------在结构体中使用泛型--------
//#[derive(Debug)]
//struct Point<T> {
//    x: T,
//    y: T,
//}
//
//#[derive(Debug)]
//struct Point2<T, U> {
//    x: T,
//    y: U,
//}
//
//fn main() {
//    let integer = Point{x: 1, y: 2};
//    println!("{:#?}", integer);
//
//    let float = Point{x: 1.1, y: 2.2};
//    println!("{:?}", float);
//
//    let a = Point2{x: 1.1, y: 'a'};
//    println!("a = {:?}", a);
//}
//
////----------在枚举中使用泛型------------
//enum Option<T> {
//    Some(T),
//    None,
//}
//
//enum Result<T, E> {
//    Ok(T),
//    Err(E)
//}
////----------------------------------------

//-----------在方法中使用泛型--------------
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn creat_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let p = Point{x:1, y: 2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    let p = Point{x:1.1, y: 2.2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    let p1 = Point2{x: 5, y: 1.1};
    let p2 = Point2{x: "hello", y: 'c'};
    let p3 = p1.creat_point(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
