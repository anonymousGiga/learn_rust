mod modA {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }

        pub fn print_a(&self) {
            println!("number: {}, name: {}", self.number, self.name);
        }
    }

    pub mod modB {
        pub fn print_B() {
            println!("B");
        }

        pub mod modC {
            pub fn print_C() {
                println!("C");
                super::print_B();
            }
        }
    }
}

//use modA::A;
use modA::A as A1;
fn main() {
    //let a = modA::A::new_a();
    //let a = A::new_a();
    let a = A1::new_a();
    a.print_a();

    let number = a.number;
    //let name = a.name;

    println!("+++++++++++++");
    modA::modB::modC::print_C();
    println!("Hello, world!");
}
