use std::io;

fn main() {
    println!("guess number");
    println!("input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("your guess: {guess}");
}
