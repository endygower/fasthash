#[cfg(test)]
#[macro_use]
extern crate quickcheck;

quickcheck! {
  fn quickcheck_32(input: (Vec<u8>, u32)) -> bool {
    let (input, seed) = input;

    let res_sys = unsafe {
      fasthash_sys::fasthash32(input.as_ptr() as _, input.len(), seed)
    };
    let res = fast_hash::hash_32(&input, seed);

    res_sys == res
  }

  fn quickcheck_64(input: (Vec<u8>, u64)) -> bool {
    let (input, seed) = input;

    let res_sys = unsafe {
      fasthash_sys::fasthash64(input.as_ptr() as _, input.len(), seed)
    };
    let res = fast_hash::hash_64(&input, seed);

    res_sys == res
  }
}
