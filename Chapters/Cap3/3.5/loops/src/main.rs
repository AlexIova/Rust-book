fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // questo viene eseguito e poi si interrompe, ritorna il valore di counter
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
