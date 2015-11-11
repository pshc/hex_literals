Simple syntax extension that converts hex strings into byte strings.

```rust
#![feature(plugin)]
#![plugin(hex_literals)]

fn main() {
    // prints "[0, 255]"
    println!("{:?}", hex!("00ff"))
}
```
