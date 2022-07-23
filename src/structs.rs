// Structs work similar to Swift structs.
// "Debug" needs to be added to let the struct data be printable
#[derive(Debug)]
struct User {
  username: String,
  active: bool,
}

pub fn structs() {
  let user1 = create_user(String::from("joe"));

  // debug (dbg) can be used to add debug output to stderr
  // "&" is added to only borrow ownership
  dbg!(&user1);
  println!("{:?}", user1); // Print with debug trait added above

  println!(
    "Username: {username}, Active: {active}",
    username = user1.username,
    active = user1.active
  );

  // ##############################################################

  // ".." works like the spread operator in JavaScript with a difference being it
  // working in reverse priority:
  // user2.active will be false because user2 is created first and then user1 is assigned to it.
  let user2 = User {
    active: false,
    ..user1
  };

  // Additionally, user1 has now "moved" and we lost ownership of it. This is only the case because
  // User contains data on the heap (username is String), resulting in it now being easily copied.

  // In this case, we don't copy over the username so we keep ownership of "user2"
  let user3 = User {
    username: String::from("jane"),
    ..user2
  };
  dbg!(user3, user2);

  // ##############################################################

  // Data in structs doesn't have to have a name and will a 0-based
  struct Point(i32, i32, i32);
  let origin = Point(0, 0, 0);
  println!("x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
}

fn create_user(username: String) -> User {
  // Implicit return is used to immediately return the value.
  User {
    username,
    active: true,
  }
}
