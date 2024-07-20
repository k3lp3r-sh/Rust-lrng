fn main() {
    let mut s = String::from("Hello, World!");

    let mut n = 10;

    let r1 = &s;

    println!("This is a immutable borrow {r1}, and this is the immutable {n}");

    let r2 = &mut s; //after this mutable borrow occurs, the pointers that point to s from the
    //mutable borrow is pushed out and only r2 is left there. somewhat similar to when the contents
    //of the file get deleted when you open them in write mode in python

    println!("This is a immutable borrow {r2}, and this is the immutable {n}");
}
