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

// --- DIADS ---
// Diads, even though it depends on the root.
naming_test!(test_c_major_diads, "C,E", "C");
naming_test!(test_d_minor_diads, "D,F", "Dm");

// --- TRIADS ---
// Basic triads
naming_test!(test_c_major, "C,E,G", "C");
naming_test!(test_d_minor, "D,F,A", "Dm");
naming_test!(test_e_flat_minor, "Eb,Gb,Bb", "E♭m");

// Diminished chords
naming_test!(test_c_sharp_diminished, "C#,E,G", "C♯dim");
naming_test!(test_d_flat_diminished, "Db,Fb,Abb", "D♭dim");

// Augmented chords
naming_test!(test_c_augmented, "C,E,G#", "Caug");
naming_test!(test_d_augmented, "D,F#,A#", "Daug");
naming_test!(test_e_flat_augmented, "Eb,G,B", "E♭aug");

// --- SEVENTH CHORDS ---
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

// --- NINTH CHORDS ---
// Basic dominant ninth
naming_test!(test_g9, "G,B,D,F,A", "G9");
naming_test!(test_a_minor_9, "A,C,E,G,B", "Am9");
naming_test!(test_c_major_9, "C,E,G,B,D", "Cmaj9");
naming_test!(test_c_minor_major_9, "C,Eb,G,B,D", "Cm(maj9)");
// omitted fifth
naming_test!(test_g9_omit_5, "G,B,F,A", "G9");
naming_test!(test_a_minor_9_omit_5, "A,C,G,B", "Am9");
naming_test!(test_c_major_9_omit_5, "C,E,B,D", "Cmaj9");
naming_test!(test_c_minor_major_9_omit_5, "C,Eb,B,D", "Cm(maj9)");
// omitting the seventh should lead to add9 chords
naming_test!(test_g_add9, "G,B,D,A", "Gadd9");
naming_test!(test_a_minor_add9, "A,C,E,B", "Amadd9");





