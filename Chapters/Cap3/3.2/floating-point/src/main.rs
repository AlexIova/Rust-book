fn main() {
    // addition
    let sum: i32 = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference: f64 = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product: i32 = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient: f64 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3; // Results in -1
    let floored: i32 = 5 / 3; // Results in 1
    println!("quotient = {}, truncated = {}, floored = {}", quotient, truncated, floored);

    // remainder
    let remainder: i32 = 43 % 5;
    println!("43 % 5 = {}", remainder);

    /*************************************/

    let t: bool = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {}, f = {}", t, f);


    let c: char = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat: char = '😻';    // È tutto in utf-8
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);



}