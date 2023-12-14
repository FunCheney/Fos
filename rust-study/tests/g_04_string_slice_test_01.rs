#[test]
fn string_fn() {
    let s = String::from("foo");
    assert_eq!("foo", s.as_str());
}
