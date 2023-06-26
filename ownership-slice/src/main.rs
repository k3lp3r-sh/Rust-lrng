fn main() {
    let s = String::from("Hello, world!");

    let mut s_ = &s[..3];

    s_ = &s[..3];
    println!("{s_}");
    
    s_ = &s[7..];
    println!("{s_}");

    s_ = &s[3..9];
    println!("{s_}");

//    s_ = &s[4..3];
    println!("{s_}");

    s_ = &s[..];
    println!("{s_}");
}
