mod ownership;

// "use" can be used like in PHP to move items into the scope
use ownership::ownership;

mod basics;

mod structs;

mod struct_methods;

mod enums;

mod option;

mod matches;

fn main() {
    println!("Learning Rust");

    basics::basics();
    ownership();
    structs::structs();
    struct_methods::struct_methods();
    enums::enums();
    option::option();
    matches::matches();
}
