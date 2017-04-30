# hex\_literals [![Build Status](https://img.shields.io/travis/pshc/hex_literals.svg)](http://travis-ci.org/pshc/hex_literals) [![Cargo](https://img.shields.io/crates/v/hex_literals.svg)](https://crates.io/crates/hex_literals)

Simple syntax extension that converts hex strings into byte strings.

```rust
#![feature(plugin)]
#![plugin(hex_literals)]

fn main() {
    // prints "[0, 255]"
    println!("{:?}", hex!("00ff"))
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
