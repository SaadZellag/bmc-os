#[cfg(test)]
use crate::should_fail;

#[test_case]
fn this_works() {
    assert_eq!(1, 1);
}

#[test_case]
fn should_fail_but_show_ok() {
    should_fail!();
    assert_eq!(0, 1);
}

#[test_case]
fn this_works_after_crash() {
    assert_eq!(0, 0);
}
