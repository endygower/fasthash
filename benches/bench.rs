#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn bench_32(b: &mut Bencher) {
  let string = b"Lorem ipsum dolor sit amet consectetur adipisicing elit";
  b.bytes = string.len() as u64;
  b.iter(|| fast_hash::hash_32(string, 0));
}

#[bench]
fn bench_c_32(b: &mut Bencher) {
  let string = b"Lorem ipsum dolor sit amet consectetur adipisicing elit";
  b.bytes = string.len() as u64;
  b.iter(|| {
    unsafe { fasthash_sys::fasthash32(string.as_ptr() as _, string.len(), 0) };
  });
}

#[bench]
fn bench_64(b: &mut Bencher) {
  let string = b"Lorem ipsum dolor sit amet consectetur adipisicing elit";
  b.bytes = string.len() as u64;
  b.iter(|| fast_hash::hash_64(string, 0));
}

#[bench]
fn bench_c_64(b: &mut Bencher) {
  let string = b"Lorem ipsum dolor sit amet consectetur adipisicing elit";
  b.bytes = string.len() as u64;
  b.iter(|| {
    unsafe { fasthash_sys::fasthash64(string.as_ptr() as _, string.len(), 0) };
  });
}
