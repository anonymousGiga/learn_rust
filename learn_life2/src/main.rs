//函数中的生命周期
//fn longest(x: &str, y: &str) -> &str {
//fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//error
//fn a_str<'a>(x: &'a str, y: &'a str) -> &'a str {
//    let r = String::from("abc");
//    r.as_str()
//}

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);

    let ss = get_str(s1.as_str(), s2.as_str());
    //let ss2 = a_str(s1.as_str(), s2.as_str());

    println!("Hello, world!");
}
