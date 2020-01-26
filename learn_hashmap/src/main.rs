//1、HashMap<K, V>
//2、创建HashMap
//3、读取
//4、遍历
//5、更新
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let k = String::from("Blue");
    if let Some(v) = scores.get(&k) { //get 返回的是一个Option
        println!("v = {}", v);
    }

    let k = String::from("Yellow");
    let v = scores.get(&k);
    match v {
        Some(value) => println!("v = {}", value),
        None => println!("None"),
    }

    println!("++++++++++++");
    //遍历:会以任意的顺序遍历出来
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    println!("++++++++++++");

    //直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 3);
    println!("{:?}", ss);

    //键不存在的时候才插入
    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    ss1.insert(String::from("three"), 3);
    ss1.entry(String::from("one")).or_insert(3);
    println!("ss1 = {:?}", ss1);

    //根据旧值来更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);

    println!("Hello, world!");
}
