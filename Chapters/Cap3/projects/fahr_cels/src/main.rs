use std::io;

fn fahr_to_cel(f: i32) -> f64 {
    ((f as f64) - 32.0) * (5.0 / 9.0)
}

fn cel_to_fahr(c: i32) -> f64 {
    ((c as f64) * (9.0 * 5.0)) + 32.0
}

fn main() {
    println!("Hello and welcome to the temperature converter!");

    loop {
        println!("Please enter the temperature you would like to convert");
        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: i32 = temp.trim().parse().expect("Please enter a number!");

        println!("Tell me what unit the temperature is in?\n\n1. F to C\n2. C to F\n");

        let mut chosen: String = String::new();
        io::stdin()
            .read_line(&mut chosen)
            .expect("Failed to read line");

        let chosen: u8 = match chosen.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result: f64;
        if chosen == 1 {
            result = fahr_to_cel(temp);
        } else if chosen == 2 {
            result = cel_to_fahr(temp);
        } else {
            println!("impossible operation, rety.");
            break;
        }

        println!("This is your result {result}");
    }
}
