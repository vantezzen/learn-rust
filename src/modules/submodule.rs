// Function are private to the current module file by default
fn some_function() {
  println!("submodule/some_function");
}

// "pub" can be used to make functions public to other modules
pub fn public_function() {
  println!("submodule/public_function");

  // Public functions can then access the private scope
  some_function();
}

pub fn other_public_function() {
  println!("submodule/other_public_function");
}

// Modules can include submodules. This will be searched in "./[Current Module name]/[Submodule name]"
mod subsubmodule;

// As with functions, modules are private by default and need to be explitily set to public
pub mod public_subsubmodule;

// Structs can be made public but each subelements needs to be made public separately
pub struct SubmoduleStruct {
  pub public_element: i32,
  private_element: i32,
}

// When having private fields on a public struct that struct cannot be initialized by outside modules!
// Instead, a public constructor should be provided instead that sets the private fields
impl SubmoduleStruct {
  pub fn new(public_element: i32) -> SubmoduleStruct {
    SubmoduleStruct {
      public_element,
      private_element: 12,
    }
  }

  // Private properties cannot be accessed so getters are needed if outside modules need to access the value
  pub fn get_private_element(self) -> i32 {
    self.private_element
  }
}

// In contrast, when an enum is public, all of its values are public by default as the Rust developers
// think its useless to have private enum properties
pub enum _SomeEnum {
  A,
  B,
}
