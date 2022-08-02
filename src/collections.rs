pub fn collections() {
  // Rust provides some collections to provide common storage methods.
  // These are always stored on the heap and thus need to use borrowing
  vectors();
  strings();
  hash_maps();
}

fn vectors() {
  // Vectors ("Vec") are arrays that have a variable length and
  // all elements have the same type
  let mut some_vec: Vec<i32> = Vec::new();

  // Like in JavaScript, push can be used to add new elements
  some_vec.push(12);

  // Square brackets can be used to get a Vector element.
  // This will panic and crash the app if the element does not exist
  println!("{}", &some_vec[0]);

  // If non-existent indexes are expected, ".get" can be used instead which
  // will return an optional instead of crashing
  match some_vec.get(1) {
    Some(num) => (),
    None => (),
  }

  // The "vec!" macro can be used to create a vector with initial values.
  // The vector content type will be inferred and doesn't need to be defined explicitly.
  // Notice the usage of square brackets.
  let other_vec = vec![String::from("Hello"), String::from("world")];

  // Elements in vectors can be iterated though using for..in
  for message in &other_vec {
    println!("{}", message);
  }

  // To store different data types, an enum can be used
  enum VecValue {
    Text(String),
    Number(i32),
  }
  let mut multi_type_vector: Vec<VecValue> = Vec::new();
  multi_type_vector.push(VecValue::Text(String::from("Hello")));
}

fn strings() {
  // Strings are also a collection
  // to_string() can be used on some data types to create a String instance. This works like String::from(...)
  let mut some_string = "hello".to_string();

  // To add another string to a string, "push_str" needs to be used
  some_string.push_str(", world");

  // This is becase the normal ".push()" is reserved for the use with a single character instead
  some_string.push('!');

  // Alternatively, the format! macro can be used. This works like print! but instead returns the value
  let formatted_string = format!("Message is: {}", some_string);
  println!("{}", formatted_string);

  // Strings cannot be indexed by integer so this is a compiler error:
  // formatted_string[0];

  // Internally, strings are saved as raw UTF-8 bytes that can contain multi-byte characters and "diacritics"
  // that aren't individual characters but rather modify the previous char (like "a" + "^" = "â")
  // Due to this, individual position accesses need to be defined as byte or character.

  let character = String::from("ते");
  for cha in character.bytes() {
    println!("Byte in character: {}", cha);
  }
  for cha in character.chars() {
    println!("Characters in character: {}", cha);

    // Output:
    // "Characters in character: त
    // Characters in character: े"
    // The char above contains a diacritics which is not handled by rusts standard library. Due to this, the single character in the string
    // produces two values. Diacritics needs to be handled using an external create.
    // Due to this, individual character accesses are not recommended by the Rust team
  }
}

fn hash_maps() {
  // Hashmaps are unloved by the Rust team and thus need to be imported manually :(
  use std::collections::HashMap;

  // HashMaps work like objects in JavaScript but are again restricted to data types
  let mut speed: HashMap<String, i32> = HashMap::new();
  // Again, HashMaps are unloved and don't have a macro to initialize them like vectors do

  speed.insert(String::from("Rust"), 100);
  speed.insert(String::from("JavaScript"), 20);

  // Values are returned as optionals
  let rust_speed = speed.get(&String::from("Rust"));
  let definately_rust_speed = rust_speed.unwrap_or_else(|| &100);
  println!("{}", definately_rust_speed);

  // "entry" returns an "Entry" enum that contains some useful helpers.
  // This will insert 200 only if there isn't an entry with that key yet
  speed.entry(String::from("C")).or_insert(200);

  // It also returns a reference that can be mutated to change the value in the map
  let c_speed = speed.entry(String::from("C")).or_insert(0);
  *c_speed += 100;
}
