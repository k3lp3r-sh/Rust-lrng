fn main() {
    let mut first_variable = "";
    let mut second_variable = "";
    let mut third_variable = "";
    let mut ctr = 3;

    let resutlt = loop {
        first_variable = "first_variable";
        second_variable = "second_variable";
        third_variable = "third_variable";
        ctr = ctr - 1;
        if ctr <= 0 {
            break (third_variable, second_variable, first_variable);
        };
    };

    // println!("{resutlt.0}, {resutlt.1}, {resutlt.2}");
    // println!("{resutlt.0}");
    println!("{:?}", resutlt)
}
