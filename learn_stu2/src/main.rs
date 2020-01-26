#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

impl Dog {
    fn get_name(&self) -> &str {
        &(self.name[..])
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    //fn get_height(&self) -> f32 {
    //    self.height
    //}

    fn show() {
        println!("oh oh oh");
    }
}

impl Dog{
    fn get_height(&self) -> f32 {
        self.height
    }
}

fn main() {
    let dog = Dog {
        name: String::from("wangcai"),
        weight: 100.0,
        height: 70.5,
    };

    println!("dog = {:#?}", dog);
    println!("name = {}", dog.get_name());
    println!("weight = {}", dog.get_weight());
    println!("height = {}", dog.get_height());

    Dog::show();

    println!("Hello, world!");
}
