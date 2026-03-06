#![allow(unused)]

fn main() {
    let x = 5; // immutable variable
    //x += 1; // this will cause a compile-time error because x is immutable
    println!("The value of x is: {}", x);
    
    let mut y = 10; // mutable variable 
    y += 1; // modifying the mutable variable
    println!("The value of y is: {}", y);
    
    y = 15; // modifying the mutable variable
    println!("The new value of y is: {}", y);

    // type inference
    let y: i32 = -1; // explicitly specifying the type of y
    println!("The value of y is: {}", y);
    let z = -1; // this will also be inferred as i32
    println!("The value of z is: {}", z);

    //shadowing
    let x: i32 = 5; // x is an integer
    let x: i32 = x + 1; // x is shadowed and now holds the value 6
    let x: bool = true; // x is shadowed again and now holds a boolean value

    //type placeholder
    let x : _  = 12345; // the type of x will be inferred from the value assigned to it
    println!("The value of x is: {}", x);

    // constants
    const NUM: i32 = 100; // constants must have a type specified
    println!("The value of NUM is: {}", NUM);

    //printing
    let x = 1;
    println!("The value of x is: {}", x); // using {} as a placeholder for the value of x
    println!("The value of NUM is: {NUM}"); // using curly braces to directly reference the constant NUM
    let z = x + x; 
    println!("{0} + {0} = {1}", x, x + x); // using positional arguments in the format string
    // debug
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x);
}