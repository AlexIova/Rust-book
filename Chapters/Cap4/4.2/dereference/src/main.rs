fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    println!("a: {a}");
    *x += 1;                 // *x on the left-side modifies the heap value,
                             //     so x points to the value 2
    println!("x: {x}");


    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    println!("b: {b}");
    
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it
    println!("c: {c}");

    /* Now implicit dereferencations */
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);

}