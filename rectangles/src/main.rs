struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Area is {}", area(&rect1));
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
