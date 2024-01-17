#[inline]
#[allow(dead_code)]
fn mix(mut h: u64) -> u64 {
  h ^= h >> 23;
  h = h.wrapping_mul(0x2127599bf4325c37);
  h ^= h >> 47;

  h
}

/// Use the 64 bit variant of fasthash to hash [u8].
///
/// # Example
///
/// ```
/// let hash_result = fast_hash::hash_64(b"hello world", 0);
/// ```
pub fn hash_64(buf: &[u8], seed: u64) -> u64 {
  const M: u64 = 0x880355f21e6d1965;
  let mut h = seed as u64 ^ (buf.len() as u64).wrapping_mul(M);

  let (chunks, tail) = buf.as_chunks();
  for &chunk in chunks {
    h ^= mix(u64::from_le_bytes(chunk));
    h = h.wrapping_mul(M);
  }

  let mut v: u64 = 0;
  for i in 0..tail.len() {
    v ^= (tail[i] as u64) << i * 8;
  }
  if !tail.is_empty() {
    h ^= mix(v);
    h = h.wrapping_mul(M);
  }

  mix(h)
}

/// Use the 32 bit variant of fasthash to hash [u8].
///
/// # Example
///
/// ```
/// let hash_result = fast_hash::hash_32(b"hello world", 0);
/// ```
pub fn hash_32(buffer: &[u8], seed: u32) -> u32 {
  let h = hash_64(buffer, seed as u64);
  (h - (h >> 32)) as u32
}
