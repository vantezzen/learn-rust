// enums work like in most other programming languages
enum Cookie {
  ChocolateChip,

  // items can have one or multiple associated values.
  // Not all items of an enum need to have the same associated values
  Filled(Filling),

  // Items can use named associated values
  Layered {
    first_layer: Filling,
    second_layer: Filling,
  },
}

// Like struct, enums can have methods.
// These will be added to each items of the enum
impl Cookie {
  fn eat(&self) {
    match self {
      Cookie::ChocolateChip => println!("I'll take another one!"),
      _ => println!("I don't like this one!"),
    }
  }
}

enum Filling {
  Caramel,
  Hazelnut,
}

pub fn enums() {
  let chocolate_chip_cookie = Cookie::ChocolateChip;
  chocolate_chip_cookie.eat();

  let layered_cookie = Cookie::Layered {
    first_layer: Filling::Caramel,
    second_layer: Filling::Hazelnut,
  };
  layered_cookie.eat();
}
