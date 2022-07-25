pub fn option() {
  // The option enum is a special enum that replaces "null" of other programming langs.
  // Instead of every variable possibly being "null", nullable items need to be explicitly
  // indicated with "Option<T>" and need to be handled separately before usage.
  // "Option" can be "Some" (i.e. has value) or "None". These values are globally available as
  // they are used often.
  let a_number = Some(5);
  let no_number: Option<i32> = None; // Type needs to be explitly annotated as it cannot be infered
  handle_number(a_number);
  handle_number(no_number);
}

fn handle_number(possibly_number: Option<i32>) {
  // Match can be used to handle the value being None or Some
  match possibly_number {
    Some(value) => println!("Optional number has {} as value", value),
    None => println!("Oh no, no number in that optional"),
  }
}
