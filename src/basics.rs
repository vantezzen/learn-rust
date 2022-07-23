// Function needs to be declared public so "main.rs" can import it
pub fn basics() {
  // println is a "macro" as indicated by the "!"
  // Macros can execute code at *compile time* to transform the code before creating the machine code.
  // This can be used to add powerful compile-time verification and optimizations like SQL-integration.
  println!("Hello, world!");

  // Immutable-by-default variables
  let x = 5;
  println!("x = {}", x);

  // Named arguments can be used to help with output
  println!(
    "{a}, this is {b}. {b}, this is {a}.",
    a = "Alice",
    b = "Bob"
  );

  // Would cause Error: cannot assign to immutable variable
  // x = 6;

  // Mutable variables must be explicitly declared as mutable
  let mut y = 5;
  y += 1;
  println!("y = {}", y);

  let num = get_a_float();
  println!("num = {}", num);

  // const can be used to indicate never changing data. This is mostly used for global constants.
  // Constants are set at compile time so only values that can be computed at compile time can be used!
  const MINUTES_PER_DAY: i32 = 24 * 60;
  println!("MINUTES_PER_DAY = {}", MINUTES_PER_DAY);

  // ##############################################################

  // String literal vs. String object:
  // String literal has fixed size at compile time and thus gets stores on the stack
  // String object has dynamic size and is stored on the heap.
  let string_literal = "hello";
  let string_object = String::from(string_literal);
  println!("{} {}", string_object, string_literal);

  // ##############################################################

  // Ownership is contained per scope. They can be explicitly declared inside functions.
  {
    // This gets executed normally like the rest of the code.
    let scope_specific = 5;
    println!("scope_specific = {}", scope_specific);
  }
  {
    // We are inside another scope so "scope_specific" is no longer available.
    // println!("x = {}", scope_specific); => Error
  }

  // ##############################################################

  // Tuple can contain a fixed number of elements with different types.
  let tuple = (5, "hello");
  println!("tuple = {:?}", tuple);

  // Array has fixed size and all elements must have the same type.
  let array = [1, 2, 3, 4, 5];
  println!("array = {:?}", array);

  // Shorthand syntax can be used to declare array with repreating values
  // This is equal to [1, 1, 1, 1, 1]
  let array_2 = [1; 5];
  println!("array_2 = {:?}", array_2);
}

// Rust convention is to use snake_case for names
fn get_a_float() -> f64 {
  let a = 1.5; // Stored as IEEE-754 double-precision floating-point number, 64 bit by default
  let b = 2.5;

  // Missing semicolon on the last line implicitly returns the value
  // Early return must be explicly called with the return keyword
  a + b
}
