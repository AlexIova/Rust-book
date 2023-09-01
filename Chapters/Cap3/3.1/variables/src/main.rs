const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    // Can be used in global scope

fn main() {
    
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Value of THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    let x: i32 = x + 1;
    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");


    let spaces = "    ";
    println!("The value of spaces is: {spaces}.");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}.");

    /* This will not compile */
    /*
     *   let mut spaces = "    ";
     *   let spaces = spaces.len();
     */

}
