use chordy::types::*;

#[test]
fn test_scale_creation() {
    let root = NoteName::new(Letter::C, Accidental::Natural);
    let scale = Scale::new(root, ScaleType::Major);

    let notes = scale.notes();
    assert!(notes.contains(&root));
}
