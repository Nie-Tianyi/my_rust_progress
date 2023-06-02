//integration tests
use auto_test;

mod common;

#[test]
fn it_works() {
    common::setup();
    assert_eq!(2 + 2, 4);
}
