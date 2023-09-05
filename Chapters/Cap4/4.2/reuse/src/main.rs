fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("second: {s}");
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("first: {} {}!", g1, g2);
    (g1, g2)
}