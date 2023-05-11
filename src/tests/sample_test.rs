#[test_case]
fn this_works() {
    assert_eq!(1, 1);
}

#[test_case]
fn this_crashes() {
    assert_eq!(0, 1);
}

#[test_case]
fn this_works_after_crash() {
    assert_eq!(0, 0);
}
