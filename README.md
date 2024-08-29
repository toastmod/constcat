# constcat 🐱

A simple macro to concatenate `const` strings in rust.

### Example
```rust
const name: &str = "MyName🐱";
const postfix: &str = "_type";
const totally_safe_name_trust_me_im_a_cat: &'static str = constcat!(name, postfix); 

#[test]
fn it_works() {
  println!("Hello, world! Looks like my cat's name is {}", totally_safe_name_trust_me_im_a_cat);
}
```

### Note
This code uses `unsafe` function `std::str::from_utf8_unchecked`, however the rest is written in safe rust.
