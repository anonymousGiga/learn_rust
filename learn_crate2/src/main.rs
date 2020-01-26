//use mylib::factory::produce_refrigerator;
////use mylib::factory::produce_refrigerator::produce_re;
////use mylib::factory::produce_washing_machine;
//use mylib::factory::produce_washing_machine as A;

use mylib::factory::*;

fn main() {
    mylib::factory::produce_refrigerator::produce_re(); //绝对路径
    produce_refrigerator::produce_re();  //使用use
    produce_washing_machine::produce_re();
    //A::produce_re();

    println!("Hello, world!");
}
