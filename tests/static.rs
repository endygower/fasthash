struct Test {
  input: &'static str,
  hash_32: u32,
  hash_64: u64,
}

#[test]
fn static_input() {
  let tests = [
    Test {
      input: "Lorem ipsum dolor sit amet, consectetur adipisicing elit",
      hash_32: 0xc441eaf0,
      hash_64: 0xaeb5ce9172f7b981,
    },
    Test {
      input: "Hello, world!",
      hash_32: 0xed771645,
      hash_64: 0x30c22fed1e394632,
    },
    Test {
      input: "",
      hash_32: 0x00000000,
      hash_64: 0x0000000000000000,
    },
    Test {
      input: "1",
      hash_32: 0x9fc278fe,
      hash_64: 0x2cc18060cc83f95e,
    },
    Test {
      input: "12",
      hash_32: 0x3398e57d,
      hash_64: 0xc708277ffaa10cfc,
    },
    Test {
      input: "123",
      hash_32: 0x331c5ca6,
      hash_64: 0x21e03ea854fc9b4e,
    },
    Test {
      input: "1234",
      hash_32: 0x2a60a258,
      hash_64: 0x261459cd5074fc25,
    },
    Test {
      input: "12345",
      hash_32: 0x1a948771,
      hash_64: 0xde905febf924e75c,
    },
    Test {
      input: "123456",
      hash_32: 0x352fb93e,
      hash_64: 0xa1061691d635cfcf,
    },
    Test {
      input: "1234567",
      hash_32: 0xae5d90db,
      hash_64: 0xbc88c5e16ae656bc,
    },
    Test {
      input: "12345678",
      hash_32: 0x72878cb6,
      hash_64: 0x2153cc5b93db5911,
    },
    Test {
      input: "123456789",
      hash_32: 0x89a855ee,
      hash_64: 0xe159b2b46b0208a2,
    },
    Test {
      input: "1234567890",
      hash_32: 0x50730364,
      hash_64: 0x9976a5b7e9e9a91b,
    },
    Test {
      input: "12345678901",
      hash_32: 0x4fd6cbbb,
      hash_64: 0xec2b8e943c025a4f,
    },
    Test {
      input: "123456789012",
      hash_32: 0x81e36da6,
      hash_64: 0xd468fcaf564c6a55,
    },
    Test {
      input: "1234567890123",
      hash_32: 0x4335f546,
      hash_64: 0x6973c761aca9bca7,
    },
    Test {
      input: "12345678901234",
      hash_32: 0x5d10c21f,
      hash_64: 0x4209e9e69f1aac05,
    },
    Test {
      input: "123456789012345",
      hash_32: 0x694c26fa,
      hash_64: 0xff8e3bfc68da62f6,
    },
    Test {
      input: "1234567890123456",
      hash_32: 0xd11284cc,
      hash_64: 0x9b8d6f576c9ff423,
    },
    Test {
      input: "12345678901234567",
      hash_32: 0x6d8f532e,
      hash_64: 0x43b52521b144784f,
    },
    Test {
      input: "123456789012345678",
      hash_32: 0xfd085bb2,
      hash_64: 0xb1396117ae41bcc9,
    },
    Test {
      input: "1234567890123456789",
      hash_32: 0xecd7760c,
      hash_64: 0xaae8d78597c04d91,
    },
    Test {
      input: "12345678901234567890",
      hash_32: 0xc9bd1218,
      hash_64: 0xd53f94939efca6ab,
    },
    Test {
      input: "123456789012345678901",
      hash_32: 0xd58dde63,
      hash_64: 0x3ebad6e41448b547,
    },
    Test {
      input: "1234567890123456789010",
      hash_32: 0xfba6554f,
      hash_64: 0x254ab61920f10b68,
    },
    Test {
      input: "€",
      hash_32: 0xc8519c3a,
      hash_64: 0x9dcf57436620f37d,
    },
    Test {
      input: "€€€€€€€€€€",
      hash_32: 0xeba2fe28,
      hash_64: 0x427aab8a2e1da9b2,
    },
  ];

  for test in &tests {
    assert_eq!(
      fast_hash::hash_32(test.input.as_bytes(), 0),
      test.hash_32,
      "hash_32 failed on string {}",
      test.input
    );

    assert_eq!(
      fast_hash::hash_64(test.input.as_bytes(), 0),
      test.hash_64,
      "hash_64 failed on string {}",
      test.input
    );
  }
}
