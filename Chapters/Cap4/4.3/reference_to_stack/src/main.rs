fn main() {
    println!("Hello, world!");
    return_a_string2();
}

// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s          // this is not safe because the stack will be cleared after the function ends
// }

// Here I give ownership outside, it's ok
fn return_a_string1() -> String {
    let s = String::from("Hello world");
    s          
}

fn return_a_string2() -> &'static str {         // this is static, is a string literal that will never change
    "Hello world"
}


use std::rc::Rc;                // Reference Counter. It will defer this to the garbage collector.
fn return_a_string3() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
