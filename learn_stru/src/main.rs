fn main() {
    //1、定义结构体
    #[derive(Debug)]
    struct User {
        name: String,
        count:String,
        nonce: u64,
        active: bool,
    }

    //2、创建结构体实例
    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("80001000"),
        nonce: 10000,
        active:true,
    };
    println!("xiaoming = {:?}", xiaoming);
    println!("xiaoming = {:#?}", xiaoming);

    //3、修改结构体字段
    let mut xiaohuang = User {
        name: String::from("xiaohuang"),
        count: String::from("80001000"),
        nonce: 10000,
        active:true,
    };

    xiaohuang.nonce = 20000;

    //4、参数名字和字段名字同名的简写方法
    let name = String::from("xiaoxiao");
    let count = String::from("89077777");
    let nonce = 200000;
    let active = false;

    //let user1 = User {
    //    name: name,
    //    count: count,
    //    nonce: nonce,
    //    active: active,
    //};

    let user1 = User {
        name,
        count,
        nonce,
        active,
    };

    //5、从其它结构体创建实例
    let user2 = User {
        name: String::from("user2"),
        ..user1
    };
    println!("name = {}", user2.name);
    println!("nonce = {}", user2.nonce);

    //6、元组结构体
    //(1)字段没有名字
    //（2）圆括号
    struct Point(i32, i32);

    let a = Point(10, 20);
    let b = Point(30, 11);

    println!("a.x = {}, a.y = {}", a.0, a.1);

    //7、没有任何字段的类单元结构体
    struct A{};

    //8、打印结构体
    println!("Hello, world!");
}
