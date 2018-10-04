extern crate rustbyexample;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rustbyexample::add_two(2));
}
