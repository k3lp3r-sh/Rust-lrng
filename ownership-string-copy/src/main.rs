fn main() {
    let s1 = "rust is a nerd";
    let s2 = String::from(s1);
    // let s1 = "some string" - this is unsafe, multiple pointers might point towards a single
    // value, meaning they can be taken by some attacker by making some other var point toward this
    // var and retrieve the value stored in that var.


    { 
    take_ownership(s2);
    
    //println!("{}", s2);
    
    }
    println!("{}", s2);
}

fn take_ownership(_string: String) {
    println!("{_string}");

}
