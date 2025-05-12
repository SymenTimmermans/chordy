use chordy::error::ParseError;

#[test]
fn test_invalid_accidental_error() {
    let err = ParseError::InvalidAccidental("x".to_string());
    assert_eq!(err.to_string(), "Invalid accidental: 'x'");
}

#[test]
fn test_invalid_note_name_error() {
    let err = ParseError::InvalidNoteName("H".to_string());
    assert_eq!(err.to_string(), "Invalid note name: 'H'");
}
