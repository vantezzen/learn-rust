pub fn matches() {
  // Match is a powerful way of handling if/else clauses
  enum Message {
    Hello,
    MinutesUntilEvent(u32),
    SomeOther,
    Messages,
  };

  let my_message = Message::Hello;
  match my_message {
    // The "unit type" ("()") can be used to indicate noop
    Message::Hello => (),

    // Curly brackets can be added to use multi-line expressions
    Message::MinutesUntilEvent(minutes) => {
      println!("{}", minutes);
      println!("Not that far away!");

      // Values can be returned here
      ()
    }

    // "match" patterns need to be exhaustive, otherwise a compiler error will be thrown.
    // This is to ensure the dev has thought about all possible values.
    // "_" can be used to indicate the value not being used.
    // Something like "_ => ()" can be used to indicate to the compiler that other cases have been
    // thought about but nothing needs to be done.
    // The catch-all should be placed at the bottom as it will catch everything that has gone though.
    _ => println!("Something else"),
  }

  // ####################################################################################

  let dice_roll: u8 = 3;

  // match returns the returned value and can be used to assign variables.
  let points = match dice_roll {
    1 => 100,
    6 => 0,

    // Instead of "_" a variable name can be used to catch all other values
    dice_value => dice_value + 1,
  };

  // ####################################################################################

  // As an alternative "if let" can be used. This is a kind of syntactical sugar for this:
  // match some_var {
  //   right_value(content) => ...,
  //   _ => ()
  // }
  // can be transformed to:
  // if let right_value(content) = some_var { ... "content" is available here }

  let might_be_number = Some(5);
  if let Some(number) = might_be_number {
    println!("Some has value {}", number);
  } else {
    println!("Some has no value");
  }
}
