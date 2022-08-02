pub fn errors() {
  // Like with null, Rust doesn't have throwable exceptions to improve knowing if a method can error
  //
  // Unrecoverable errors will crash and panic the app. It is expected that these are unwanted and
  // may be security holes. Trying to access out of index on array will panic.
  // Alternatively, a panic can be manually caused using
  // panic!("This failed");
  //
  // Recoverable errors are to be expected and handled.
  // Functions that might error can return a "Result" enum that can be "Ok(T)" or "Err(E)"
  use std::fs::File;
  use std::io::ErrorKind;
  let f = File::open("doesntExist.txt");
  let _f = match f {
    Ok(_file) => (),
    Err(err) => {
      println!("Can't open file: {}", err);
      if err.kind() == ErrorKind::NotFound {
        println!("as expected");
      }

      ()
    }
  };

  // In functions, errors can be propagated to higher functions using "?"
  // These can be handled like normal errors
  propagates_error().unwrap_or("".to_string());
}

fn propagates_error() -> Result<String, std::io::Error> {
  use std::fs::File;
  use std::io::Read;

  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}
