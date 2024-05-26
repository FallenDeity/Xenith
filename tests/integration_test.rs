mod common;

use xenith;

#[test]
fn it_adds_two() {
    assert_eq!(4, xenith::add_two(2));
}
