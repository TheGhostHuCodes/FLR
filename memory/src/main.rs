fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1;
    // Use variable after move below will not compile.
    // println!("{}, world!", s1);
    println!("{}, world!", s2);
}
