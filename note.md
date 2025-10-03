# About 
- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
- Cargo is Rust’s build system and package manager

# Basic Commands
- cargo build: Compile the current project
- cargo run: Compile and run the current project
- cargo check: Check the current project for errors without producing an executable
- cargo new <project_name>: Create a new Rust project

# compile single files
```bash
rustc <file_name>.rs
```

# examples

let - is used to bind a value to a variable. By default, variables are immutable, meaning their values cannot be changed after they are set. To make a variable mutable, you can use the mut keyword.

mut - is used to make a variable mutable, allowing its value to be changed after it has been initially set.
````
let two = 2;
let mut three = 3;
three = 4; // This is allowed because 'three' is mutable
````

# Macros
- Macros are a way of writing code that writes other code, which is known as metaprogramming. In Rust, macros are defined using the macro_rules! construct.
- examples:
```
println!("Hello, world!"); // This is a macro that prints text to the console
vec![1, 2, 3]; // This is a macro that creates a vector
println!("{} + {} = {}", 1, 2, 1 + 2); // This is a macro that prints formatted text to the console
```

# Control flow

- if else statements
```

let a = 99;
if a < 100 {
    println!("a is less than 100");
} else {
    println!("a is greater than or equal to 100");
}
```

# loops
```
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break;
    }
}