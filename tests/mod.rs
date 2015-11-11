#![feature(plugin)]
#![plugin(hex_literals)]

#[test]
fn hex() {
    assert_eq!(hex!("0123456789 abcdef"), b"\x01\x23\x45\x67\x89\xab\xcd\xef");
}
