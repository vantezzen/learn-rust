mod ownership;
use ownership::ownership;

mod basics;
use basics::basics;

mod structs;
use structs::structs;

fn main() {
    println!("Learning Rust");

    basics();
    ownership();
    structs();
}
