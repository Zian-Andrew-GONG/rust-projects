use adder;
mod common;
#[test]
fn two_adds_two() {
    common::set_up();
    assert_eq!(4, adder::add(2, 2));
}
