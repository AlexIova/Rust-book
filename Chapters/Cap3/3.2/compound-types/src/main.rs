fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;        // destructuring

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("{} {} {}", tup.0, tup.1, tup.2);


    println!("\n*****************\n");
    /****************************/

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
                                "August", "September", "October", "November", "December"];
    println!("{} {} {} {} {} {} {} {} {} {} {} {}",
               months[0], months[1], months[2], months[3], months[4], months[5], months[6],
               months[7], months[8], months[9], months[10], months[11]);

    let a = [3; 5];     // Conterrà lo stesso valore `3` cinque volte
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    let first = a[0];
    let second = a[1];
    println!("{} {}", first, second);






}