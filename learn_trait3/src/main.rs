//fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
fn largest<T> (list: &[T]) -> T 
    where T: PartialOrd + Copy
{
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {
    let number_list = vec![1, 2, 23, 34, 8, 100];
    //let max_number = largest_i32(&number_list);
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);

    let char_list= vec!['a', 'y', 'b'];
    //let max_char = largest_char(&char_list);
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);
    println!("Hello, world!");
}
