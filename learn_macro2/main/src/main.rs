use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Main;

fn main() {
    Main::hello_macro();
    println!("Hello, world!");
}
