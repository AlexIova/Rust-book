fn main() {
    
    let mut x: u8 = 0;

    loop{
        println!("Valore di x per adesso: {x}");
        x = x + 1;
    }

    /* Se faccio `cargo run` e basta essendo in debug mi da un panic in overflow */
    /* Se faccio `cargo run --release` vengono tolti i check e mi fa wrapping */

}
