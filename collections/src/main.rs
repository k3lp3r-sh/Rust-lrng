use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let mut hello = String::from("Hola");

    hello.push_str("amigo");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    for c in s.chars() {
        println!("{c}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 12);
    scores.insert(String::from("Yello"), 14);

    let score = scores.get(&String::from("Red")).unwrap();

    println!("{score}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let count = scores.entry(String::from("Red")).or_insert(0);
    *count += 1;

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
