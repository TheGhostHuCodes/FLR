fn main() {
    let x = 1;

    if x == 1 {
        let a = 10;
        println!("a = {}", a);
    } // The variable a falls out of scope here.
}
