fn main() {
    let copy_of_string = dangle();
    println!("{}", copy_of_string);
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
