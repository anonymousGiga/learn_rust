//1、trait_bound语法
//2、指定多个trait bound
//3、返回 trait的类型
//fn print_information(item: impl GetInformation) { //直接作为参数的写法
//fn print_information<T: GetInformation>(item: T){ //使用trait bound的写法
//    println!("name = {}", item.get_name());
//    println!("age = {}", item.get_age());
//}

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

////写法一：
//fn print_information<T: GetName+GetAge>(item: T){ //使用trait bound的写法
//    println!("name = {}", item.get_name());
//    println!("age = {}", item.get_age());
//}

//写法二：
fn print_information<T>(item: T)
    where T: GetName+GetAge
{ //使用trait bound的写法
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn produce_item_with_age() -> impl GetAge {
    Student {
        name: String::from("xiaoming"),
        age: 15,
    }
}

#[derive(Debug)]
pub struct Teacher{
    pub name: String,
    pub age: u32,
}

impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

//错误的用法
//fn produce_item_with_age2() -> impl GetAge {
//    let is = true;
//
//    if is {
//        Student {
//            name: String::from("xiaoming"),
//            age: 15,
//        }
//    } else {
//        Teacher {
//            name: String::from("xiaohuang"),
//            age: 15,
//        }
//    }
//}

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    print_information(s);

    let s = produce_item_with_age();
    
    println!("Hello, world!");
}
