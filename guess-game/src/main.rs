use std::io;

fn main() {
    println!("guess the number");
    println!("give your input: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("Your guess was: {guess}");

}
