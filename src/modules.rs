// Modules can be defined using the "mod" keyword
// This only needs to be done once in the whole code to let the compiler know that this file should
// be included. From then on, the module can simply be used or brought into context with "use"
mod submodule;

// The exported function by submodule can be accessed by using "submodule::public_function"
// or by bringing elements into the current scope
// use submodule::public_function;

// Multiple "use" can be put into one expression using curly brackets
use submodule::{other_public_function, public_function};

pub fn modules() {
  public_function();
  other_public_function();

  // Public submodules can be accessed like using file paths but using "::" instead of "/"
  submodule::public_subsubmodule::public_subsubmodule_function();

  {
    // "use" are scoped to the current scope
    use submodule::public_subsubmodule::public_subsubmodule_function;

    public_subsubmodule_function();
  }
  // This would be a compiler error as the scope is different here:
  // public_subsubmodule_function();

  // The "submodule::" accesses are relative paths as they start from the current module context.
  // Alternatively, an absolute path can be used by starting with "crate::" for the current crate
  // context. "crate::"  works like using "/" in paths
  crate::modules::submodule::public_function();

  let some_struct = submodule::SubmoduleStruct::new(21);
  let public_element = some_struct.public_element;
  let private_element = some_struct.get_private_element();
  println!("{}; {}", public_element, private_element);
}
