struct student {
    rolno: i32,
    name: String,
    mark: f64
}

struct teacher {
    Name: &'a str,
    Num: i32
}
fn main() {
    
    let Alice = student {
        rolno: 1,
        name: String::from("Alice"),
        mark: 6.9
    }; 
    
    println!("{}", Alice.mark);


    println!("--------------");

    let pig = teacher {
        Name: &"cancer", //throws error because there is not borrow addr
        Num: 10
    }; 

    println!("{}", pig.Name);

}


