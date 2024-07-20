use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    let integr: i32 = 0;
    println!("{}", size_of_val(&integr))
}
