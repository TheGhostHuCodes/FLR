use std::fs::File;

fn main() {
    let _f = File::open("hello.txt").expect("File was not found!");

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("File was not found!");
    //     }
    // };
}
