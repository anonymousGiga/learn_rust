#![crate_type = "staticlib"]
#[no_mangle]
pub extern fn foo() {
    println!("use rust");
}
