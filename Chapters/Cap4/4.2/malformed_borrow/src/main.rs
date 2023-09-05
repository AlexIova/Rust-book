fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}


/* Dopo aver fatto `cargo run` quello che dice il compilatore: */

// warning: unused variable: `s`
//  --> src/main.rs:5:9
//   |
// 5 |     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
//   |         ^ help: if this is intentional, prefix it with an underscore: `_s`
//   |
//   = note: `#[warn(unused_variables)]` on by default

// error[E0382]: borrow of moved value: `m1`
//  --> src/main.rs:5:30
//   |
// 2 |     let m1 = String::from("Hello");
//   |         -- move occurs because `m1` has type `String`, which does not implement the `Copy` trait
// 3 |     let m2 = String::from("world");
// 4 |     greet(m1, m2);
//   |           -- value moved here
// 5 |     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
//   |                              ^^ value borrowed here after move
//   |
// note: consider changing this parameter type in function `greet` to borrow instead if owning the value isn't necessary
//  --> src/main.rs:8:14
//   |
// 8 | fn greet(g1: String, g2: String) {
//   |    -----     ^^^^^^ this parameter takes ownership of the value
//   |    |
//   |    in this function
//   = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 4 |     greet(m1.clone(), m2);
//   |             ++++++++

// error[E0382]: borrow of moved value: `m2`
//  --> src/main.rs:5:34
//   |
// 3 |     let m2 = String::from("world");
//   |         -- move occurs because `m2` has type `String`, which does not implement the `Copy` trait
// 4 |     greet(m1, m2);
//   |               -- value moved here
// 5 |     let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
//   |                                  ^^ value borrowed here after move
//   |
// note: consider changing this parameter type in function `greet` to borrow instead if owning the value isn't necessary
//  --> src/main.rs:8:26
//   |
// 8 | fn greet(g1: String, g2: String) {
//   |    -----                 ^^^^^^ this parameter takes ownership of the value
//   |    |
//   |    in this function
//   = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 4 |     greet(m1, m2.clone());
//   |                 ++++++++

// For more information about this error, try `rustc --explain E0382`.
// warning: `malformed_borrow` (bin "malformed_borrow") generated 1 warning
// error: could not compile `malformed_borrow` (bin "malformed_borrow") due to 2 previous errors; 1 warning emitted