fn main() {
    let mut var = 10;
    let var = read_int();
    println!("Your name is: {}", input);

    if var % 10 == 0 {
        println!("div by 10")
    } else {
        println!("can't div by 10");
    }
}

fn read_int() -> i32 {
    let mut input;
    std::io::stdin()
        .read_line(&mut input)
        .expect("cannot read user input");
    input
}
