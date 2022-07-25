struct Engine {
  active: bool,
  power: u8,
}

// Implementations extend Structs with methods
impl Engine {
  // Static method impls are often used to create a "constructor"-like function for it
  fn new(power: u8) -> Engine {
    Engine {
      active: false,
      power,
    }
  }

  // Similar to Python, a reference to self can be requires as the first arg to
  // run as an instance method
  fn speed(&self) -> u8 {
    if self.active {
      return self.power;
    }
    0
  }
}

// Multiple impl blocks can be used without them overriding each other
impl Engine {
  // Mutatable self instance can be used to modify the instance
  fn turn_on(&mut self) {
    self.active = true;
  }
}

pub fn struct_methods() {
  // Static methods can be accessed using "::" on the struct name
  let engine1 = Engine::new(10);

  println!("Engine 1 speed: {}", engine1.speed());
  // engine1 is not mutatable so the mutating method "turn_on" would cause a compiler error
  // engine1.turn_on();

  let mut engine2 = Engine::new(100);
  println!("Engine 2 speed: {}", engine2.speed());
  engine2.turn_on();
  println!("Engine 2 speed: {}", engine2.speed());
}
