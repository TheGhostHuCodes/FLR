fn main() {
    let mut s = String::from("Hello"); // The variable s comes into scope.

    borrow_ownership_and_mutate(&mut s); // s's value borrowed into this
                                         // function.

    println!("{}", s);
    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the function, but i32 types are
                   // copied, so it’s okay to still use x afterward.
} // Here, x goes out of scope, then s.

fn borrow_ownership_and_mutate(some_string: &mut String) {
    // some_string comes into scope.
    some_string.push_str(", World!");
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
