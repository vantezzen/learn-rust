pub fn ownership() {
  // This is a immutable string that is owned by this function.
  let s1 = String::from("hello");

  // This transfers ownership of the string to the other function
  takes_string_ownership(s1);

  // We transferred ownership and can't use s1 here anymore. This would be a compiler error:
  // println!("{}", s1);

  // ##############################################################

  // Ownership only applies to data on the heap.
  // If the value is stored on the stack (i.e. known fixed size at build time), Rust will copy the data.
  // (Implements "drop" vs implements "copy")
  let s2 = 123; // i32 by default
  takes_int_ownership(s2);

  // This still works
  println!("{}", s2);

  // ##############################################################

  // Borrowing can be used to temporarily give access to a value without transferring
  // complete ownership. Borrowing is indicated with an ampersand.
  // The value will be borrowed as a "reference".
  let s3 = String::from("hello");
  borrows_string(&s3);
  println!("{}", s3);

  // ##############################################################

  // Mutable borrows can be used to modify the value.
  let mut s4 = String::from("hello");
  // As with variables, mutable borrows must be explicitly declared.
  borrows_mutable_string(&mut s4);
  println!("{}", s4);

  // ##############################################################

  // Unlimited number of immutable borrows can be used but
  // only one mutable borrow without additional immutable borrows can be used at a time.
  let mut s5 = String::from("hello");
  {
    // This is ok
    let _ref1 = &s5;
    let _ref2 = &s5;
    let _ref3 = &s5;

    // This would now cause a compiler error as we have remaining immutable borrows open.
    // let _ref4 = &mut s5;
  }
  {
    // We are now inside a new scope so we can borrow mutable again.
    let _ref5 = &mut s5;
  }

  // ##############################################################

  // Special reference is the "slice" which is a view into a part of the data.
  // It is a pointer to the start of the data and a length.
  let s6 = String::from("hello");
  let slice = &s6[1..3]; // = "el"
  println!("slice = {}", slice);

  // Shorthand syntax for slices:
  let _slice2 = &s6[..3]; // = 0..3
  let _slice3 = &s6[3..]; // = 3..s6.len()
  let _slice4 = &s6[..]; // = 0..s6.len() i.e. complete slice
}

fn takes_string_ownership(value: String) {
  println!("{}", value);

  // We know leave the function scope which results in "drop" being called on the value
  // and the value being freed from the heap.
}

fn takes_int_ownership(value: i32) {
  println!("{}", value);
}

fn borrows_string(value: &String) {
  println!("{}", value);
}

fn borrows_mutable_string(value: &mut String) {
  value.push_str("!");
}
