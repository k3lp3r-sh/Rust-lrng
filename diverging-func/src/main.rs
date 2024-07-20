use core::panic;

#[allow(unused_imports)]
use std::io;

fn main() {
    let mut ip = String::new();

    std::io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read line");

    let num: i32 = match ip.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer!");
            return;
        }
    };

    match num {
        1 => {
            diverging_through_panic();
        }

        2 => {
            diverging_through_unimplemented();
        }

        3 => {
            diverging_through_todo();
        }

        _ => {
            println!("enter a valid number between 1 and 3")
        }
    }
}

#[allow(dead_code)]
fn diverging_through_panic() -> ! {
    panic!("panic")
}

#[allow(dead_code)]
fn diverging_through_unimplemented() -> ! {
    unimplemented!("unimplemented!")
}

#[allow(dead_code)]
fn diverging_through_todo() -> ! {
    todo!("todo")
}
