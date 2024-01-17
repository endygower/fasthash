#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn sanity_check() {
    unsafe {
      let test = b"hello";
      assert_eq!(fasthash32(test.as_ptr() as _, test.len(), 0), 4263912133);
    }
  }
}
