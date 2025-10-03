
use std::io;

fn main() {
    println!("Hello, world!");
    guessing_game();
}

fn guessing_game(){
    println!("Guess the number!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    let sum = add(5, 6);
    println!("{} is the number", sum);

    let compare = guess.trim().parse::<i32>().unwrap();
    compare_number(compare);

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn compare_number(a: i32){
    if a > 52{
        println!("This number is greater than 52")
    }
    else {
        println!("This umber is less than 52")
    }
}
