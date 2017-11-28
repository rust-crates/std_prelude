extern crate std_prelude;
use std_prelude::*;

#[test]
fn it_works() {
    let mut h = HashMap::new();
    h.insert("hello", 4);

    assert_eq!("hello", str::from_utf8(b"hello").unwrap());
    assert_eq!(42, usize::from_str("42").unwrap());
    assert!(usize::MAX > 2);

    // assert!(i128::MAX > 2);
}
