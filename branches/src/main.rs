fn main() {
    let a = 6;

    if a < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    if a % 4 == 0 {
        println!("a is divisible by 4");
    }
    if a % 3 == 0 {
        println!("a is divisible by 3");
    }
    if a % 2 == 0 {
        println!("a is divisible by 2");
    }

    if a % 4 != 0 && a % 3 != 0 && a % 2 != 0 {
        println!("a is not divisible by 4, 3, or 2")
    }
}
