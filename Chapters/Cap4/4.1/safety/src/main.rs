fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn make_and_drop() {
    let a_box = Box::new(5);    // questo verrà deallocato una volta uscito dallo scope
    println!("a_box: {a_box}");
}

fn main() {
    let x = true;
    read(x);

    let a = Box::new(15);       // Box == heap
    let b = a;                  // adesso b possiede il Box
    let c = Box::new(15);

    println!("canm't use a since it moved! | b: {}, c: {}", b, c);

    println!("\n********\n");

    let a_num = 4;
    println!("a_num: {a_num}");
    make_and_drop();

}

