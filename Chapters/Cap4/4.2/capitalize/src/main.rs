fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn main() {

    let mut vettore: Vec<char> = vec!['h','e','l','l','o'];
    println!("{:?}", vettore);
    ascii_capitalize(&mut vettore);


}