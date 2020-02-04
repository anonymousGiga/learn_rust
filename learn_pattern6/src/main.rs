struct Point{
    x: i32, 
    y: i32,
}

fn main() {
    let ((a, b), Point{x, y}) = ((1, 2), Point{x: 3, y: 4});
    println!("a: {}, b: {}, x: {}, y: {}", a, b, x, y);
    println!("Hello, world!");
}
