fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    print!("r: {r}");
}
