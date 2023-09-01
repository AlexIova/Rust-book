fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    let guess_wrong: u32 = "42i".parse().expect("Not a number!");
    println!("guess: {}", guess_wrong);
}
