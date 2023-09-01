fn fibonacci(n: i128) -> i128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn main() {
    
    let result: i128 = fibonacci(45);
    println!("Result: {result}")

}
