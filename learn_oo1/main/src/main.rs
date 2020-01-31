use getaver;

fn main() {
    let mut a = getaver::AverCollect::new();

    a.add(1);
    println!("average = {}", a.average());

    a.add(2);
    println!("average = {}", a.average());

    a.add(3);
    println!("average = {}", a.average());

    a.remove();
    println!("average = {}", a.average());

    println!("Hello, world!");
}
