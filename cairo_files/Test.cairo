#[test]
fn test_assert_true() {
    // Asserts that true
    assert(true, 'assert(true)');
}

#[test]
#[should_panic]
fn test_assert_false() {
    assert(false, 'assert(false)');
}