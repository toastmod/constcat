
// mod loose;

// #[macro_use]
// mod method;
// use method::*;

// mod variadic;

// struct Object {
//     data: u8
// }

// impl NamedTrait for Object {
//     const OBJNAME: &'static str = "Object";
// }

// constcat!(Object {
//     OBJNAME_TYPE = _types
// });

use varyingpostfix::test_this;

mod varyingpostfix;

fn main() {
    // test_this()
    // println!("Hello, world! Looks like my object's name is {}", Object::OBJNAME_TYPE);
    test_this()
}
