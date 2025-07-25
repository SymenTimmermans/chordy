// This test file verifies compile-time validation works correctly
// Uncomment these lines to test compilation failures:

// use chordy::key;

// This should fail: Invalid note letter
// const INVALID_NOTE: chordy::Key = key!("H");

// This should fail: Invalid syntax (extra characters after 'm')
// const INVALID_SYNTAX: chordy::Key = key!("Cm7");

// This should fail: Empty string
// const EMPTY: chordy::Key = key!("");

// This should fail: Just 'm'
// const JUST_M: chordy::Key = key!("m");

#[test]
fn test_compile_time_validation_placeholder() {
    // This test exists so cargo test doesn't complain about no tests
    assert!(true);
}
