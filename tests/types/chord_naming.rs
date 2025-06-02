use chordy::prelude::*;

macro_rules! naming_test {
    ($name:ident, $chord_str:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let chord: Chord = $chord_str.parse().unwrap();
            let notes = chord.intervals().iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            assert_eq!(
                chord.abbreviated_name(), 
                $expected,
                "Chord with notes {} should be named `{}`", 
                notes,
                $expected
            );
        }
    };
}
// Diads, even though it depends on the root.
naming_test!(test_c_major_diads, "C,E", "C");
naming_test!(test_d_minor_diads, "D,F", "Dm");


// Basic triads
naming_test!(test_c_major, "C,E,G", "C");
naming_test!(test_d_minor, "D,F,A", "Dm");
naming_test!(test_e_flat_aug, "Eb,G,B", "E♭aug");

// Diminished chords
naming_test!(test_c_sharp_diminished, "C#,E,G", "C♯dim");
naming_test!(test_d_flat_diminished, "Db,Fb,Abb", "D♭dim");




// Seventh chords
naming_test!(test_g7, "G,B,D,F", "G7");
naming_test!(test_a_minor_7, "A,C,E,G", "Am7");
naming_test!(test_c_major_7, "C,E,G,B", "Cmaj7");
naming_test!(test_c_minor_major_7, "C,Eb,G,B", "Cm(maj7)");

// Test with omitted fifth
naming_test!(test_g7_omit_5, "G,B,F", "G7");
naming_test!(test_a_minor_7_omit_5, "A,C,G", "Am7");
naming_test!(test_c_major_7_omit_5, "C,E,B", "Cmaj7");
naming_test!(test_c_minor_major_7_omit_5, "C,Eb,B", "Cm(maj7)");


