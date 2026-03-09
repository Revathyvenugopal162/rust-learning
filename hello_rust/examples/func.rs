#![allow(unused)]

fn add(a: u32, b: u32) -> u32 {
    let c = a + b;
    println!("Adding {} and {} gives {}", a, b, c);
    return c;  
}

fn main() {
    let result = add(5, 10);
    println!("The result of adding 5 and 10 is: {}", result);
    let string = print("Hello, world!".to_string());

    // type conversion
    let i: i32 = -1;
    let u : u32 = i as u32;
    println!("The value of u is: {}", u);

    // min max
    let i_max = i32::MAX;
    let u_min = u32::MIN;
    println!("The maximum value of i32 is: {}", i_max);
    println!("The minimum value of u32 is: {}", u_min);
}

fn print(s: String) {
    println!("{}", s);
}
