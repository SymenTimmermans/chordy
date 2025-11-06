use chordy::{PitchClassSet, note, pitch};

#[test]
fn test_pitch_class_set_creation() {
    // Create from note names
    let notes = vec![note!("C"), note!("E"), note!("G")];
    let pc_set = PitchClassSet::new(&notes);
    assert_eq!(pc_set.len(), 3);
    assert!(pc_set.contains(note!("C")));
    assert!(pc_set.contains(note!("E")));
    assert!(pc_set.contains(note!("G")));
    assert!(!pc_set.contains(note!("A")));

    // Create from pitches (should ignore octave)
    let pitches = vec![pitch!("C4"), pitch!("E4"), pitch!("G4"), pitch!("C5")];
    let pc_set = PitchClassSet::from_pitches(&pitches);
    assert_eq!(pc_set.len(), 3); // C5 is same pitch class as C4
    assert!(pc_set.contains(note!("C")));
    assert!(pc_set.contains(note!("E")));
    assert!(pc_set.contains(note!("G")));
}

#[test]
fn test_pitch_class_set_empty() {
    let empty_set = PitchClassSet::new(&[]);
    assert!(empty_set.is_empty());
    assert_eq!(empty_set.len(), 0);

    let non_empty_set = PitchClassSet::new(&[note!("C")]);
    assert!(!non_empty_set.is_empty());
}

#[test]
fn test_pitch_class_set_to_vec() {
    let notes = vec![note!("E"), note!("G"), note!("C")];
    let pc_set = PitchClassSet::new(&notes);
    let vec = pc_set.to_vec();

    // Should be sorted
    assert_eq!(vec, vec![note!("C"), note!("E"), note!("G")]);
}

#[test]
fn test_pitch_class_set_normal_form() {
    // Test major triad in different orderings
    let notes1 = vec![note!("C"), note!("E"), note!("G")];
    let pc_set1 = PitchClassSet::new(&notes1);
    assert_eq!(pc_set1.normal_form(), vec![note!("C"), note!("E"), note!("G")]);

    let notes2 = vec![note!("E"), note!("G"), note!("C")];
    let pc_set2 = PitchClassSet::new(&notes2);
    assert_eq!(pc_set2.normal_form(), vec![note!("C"), note!("E"), note!("G")]);

    let notes3 = vec![note!("G"), note!("C"), note!("E")];
    let pc_set3 = PitchClassSet::new(&notes3);
    assert_eq!(pc_set3.normal_form(), vec![note!("C"), note!("E"), note!("G")]);

    // Test diminished triad
    let notes4 = vec![note!("C"), note!("Eb"), note!("Gb")];
    let pc_set4 = PitchClassSet::new(&notes4);
    assert_eq!(pc_set4.normal_form(), vec![note!("C"), note!("Eb"), note!("Gb")]);

    // Test with enharmonic equivalents
    let notes5 = vec![note!("C"), note!("D#"), note!("F#")];
    let pc_set5 = PitchClassSet::new(&notes5);
    assert_eq!(pc_set5.normal_form(), vec![note!("C"), note!("D#"), note!("F#")]);

    // Test empty set
    let empty_set = PitchClassSet::new(&[]);
    assert!(empty_set.normal_form().is_empty());

    // Test single element
    let single_set = PitchClassSet::new(&[note!("C")]);
    assert_eq!(single_set.normal_form(), vec![note!("C")]);

    // Test two elements
    let two_set = PitchClassSet::new(&[note!("E"), note!("C")]);
    assert_eq!(two_set.normal_form(), vec![note!("C"), note!("E")]);
}

#[test]
fn test_pitch_class_set_prime_form() {
    // Test major triad
    let major_triad = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    assert_eq!(major_triad.prime_form(), vec![0, 4, 7]);

    // Test minor triad
    let minor_triad = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("G")]);
    assert_eq!(minor_triad.prime_form(), vec![0, 3, 7]);

    // Test diminished triad
    let diminished_triad = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("Gb")]);
    assert_eq!(diminished_triad.prime_form(), vec![0, 3, 6]);

    // Test augmented triad
    let augmented_triad = PitchClassSet::new(&[note!("C"), note!("E"), note!("G#")]);
    assert_eq!(augmented_triad.prime_form(), vec![0, 4, 8]);

    // Test dominant seventh
    let dominant_seventh = PitchClassSet::new(&[note!("C"), note!("E"), note!("G"), note!("Bb")]);
    println!("DEBUG: Dominant seventh notes: {:?}", dominant_seventh.to_vec());
    println!("DEBUG: Dominant seventh normal form: {:?}", dominant_seventh.normal_form());
    println!("DEBUG: Dominant seventh prime form: {:?}", dominant_seventh.prime_form());
    assert_eq!(dominant_seventh.prime_form(), vec![0, 4, 7, 10]);

    // Test major seventh
    let major_seventh = PitchClassSet::new(&[note!("C"), note!("E"), note!("G"), note!("B")]);
    assert_eq!(major_seventh.prime_form(), vec![0, 4, 7, 11]);

    // Test empty set
    let empty_set = PitchClassSet::new(&[]);
    assert!(empty_set.prime_form().is_empty());

    // Test single element
    let single_set = PitchClassSet::new(&[note!("C")]);
    assert_eq!(single_set.prime_form(), vec![0]);
}

#[test]
fn test_pitch_class_set_interval_vector() {
    // Test major triad
    let major_triad = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    assert_eq!(major_triad.interval_vector(), [0, 0, 1, 1, 1, 0]);

    // Test minor triad
    let minor_triad = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("G")]);
    assert_eq!(minor_triad.interval_vector(), [0, 0, 1, 1, 1, 0]);

    // Test diminished triad
    let diminished_triad = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("Gb")]);
    println!("DEBUG: Diminished triad notes: {:?}", diminished_triad.to_vec());
    println!("DEBUG: Interval vector: {:?}", diminished_triad.interval_vector());
    // Correct interval vector for diminished triad [0, 3, 6]:
    // - Interval class 3: 2 occurrences (0-3, 3-6)
    // - Interval class 6: 1 occurrence (0-6)
    assert_eq!(diminished_triad.interval_vector(), [0, 0, 2, 0, 0, 1]);

    // Test augmented triad
    let augmented_triad = PitchClassSet::new(&[note!("C"), note!("E"), note!("G#")]);
    // Correct interval vector for augmented triad [0, 4, 8]:
    // - Interval class 4: 3 occurrences (0-4, 4-8, 8-0 which is 12-8=4)
    assert_eq!(augmented_triad.interval_vector(), [0, 0, 0, 3, 0, 0]);

    // Test dominant seventh
    let dominant_seventh = PitchClassSet::new(&[note!("C"), note!("E"), note!("G"), note!("Bb")]);
    // Correct interval vector for dominant seventh [0, 4, 7, 10]:
    // - Class 2: 1 occurrence (C-Bb: 10 semitones → 12-10=2)
    // - Class 3: 2 occurrences (E-G, G-Bb)
    // - Class 4: 1 occurrence (C-E)
    // - Class 5: 1 occurrence (C-G)
    // - Class 6: 1 occurrence (E-Bb)
    assert_eq!(dominant_seventh.interval_vector(), [0, 1, 2, 1, 1, 1]);

    // Test major seventh
    let major_seventh = PitchClassSet::new(&[note!("C"), note!("E"), note!("G"), note!("B")]);
    // Correct interval vector for major seventh [0, 4, 7, 11]:
    // - Class 1: 1 occurrence (C-B: 11 semitones → 12-11=1)
    // - Class 3: 1 occurrence (E-G)
    // - Class 4: 2 occurrences (C-E, G-B)
    // - Class 5: 2 occurrences (C-G, E-B)
    assert_eq!(major_seventh.interval_vector(), [1, 0, 1, 2, 2, 0]);

    // Test whole-tone scale (6 notes)
    let whole_tone = PitchClassSet::new(&[
        note!("C"), note!("D"), note!("E"), note!("F#"), note!("G#"), note!("A#")
    ]);
    assert_eq!(whole_tone.interval_vector(), [0, 6, 0, 6, 0, 3]);

    // Test empty set
    let empty_set = PitchClassSet::new(&[]);
    assert_eq!(empty_set.interval_vector(), [0, 0, 0, 0, 0, 0]);

    // Test single element
    let single_set = PitchClassSet::new(&[note!("C")]);
    assert_eq!(single_set.interval_vector(), [0, 0, 0, 0, 0, 0]);

    // Test two elements
    let two_set = PitchClassSet::new(&[note!("C"), note!("E")]);
    // C to E is 4 semitones (major third) → interval class 4
    assert_eq!(two_set.interval_vector(), [0, 0, 0, 1, 0, 0]);
}

#[test]
fn test_pitch_class_set_operations() {
    let set1 = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    let set2 = PitchClassSet::new(&[note!("E"), note!("G"), note!("B")]);
    let set3 = PitchClassSet::new(&[note!("C"), note!("E")]);

    // Test subset
    assert!(set3.is_subset_of(&set1));
    assert!(!set1.is_subset_of(&set3));
    assert!(!set1.is_subset_of(&set2));

    // Test intersection
    let intersection = set1.intersection(&set2);
    assert_eq!(intersection.to_vec(), vec![note!("E"), note!("G")]);

    // Test complement
    let complement = set1.complement();
    assert_eq!(complement.len(), 9); // 12 total pitch classes - 3 in set1 = 9
    assert!(!complement.contains(note!("C")));
    assert!(!complement.contains(note!("E")));
    assert!(!complement.contains(note!("G")));
    assert!(complement.contains(note!("C#")));
    assert!(complement.contains(note!("D")));
    assert!(complement.contains(note!("F")));
    assert!(complement.contains(note!("A")));
    assert!(complement.contains(note!("B")));

    // Test complement of empty set
    let empty_set = PitchClassSet::new(&[]);
    let full_complement = empty_set.complement();
    assert_eq!(full_complement.len(), 12);

    // Test complement of full set
    let all_notes: Vec<_> = (0..12)
        .map(|pc| chordy::NoteName::from_fifths(pc))
        .collect();
    let full_set = PitchClassSet::new(&all_notes);
    println!("Full set notes: {:?}", full_set.to_vec());
    let empty_complement = full_set.complement();
    println!("Complement of full set size: {}", empty_complement.len());
    println!("Complement of full set notes: {:?}", empty_complement.to_vec());
    assert!(empty_complement.is_empty());
}

#[test]
fn test_pitch_class_set_chord_recognition() {
    // Test major triad
    let major_triad = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    assert!(major_triad.is_major_triad());
    assert!(!major_triad.is_minor_triad());
    assert!(!major_triad.is_diminished_triad());
    assert!(!major_triad.is_augmented_triad());

    // Test minor triad
    let minor_triad = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("G")]);
    assert!(!minor_triad.is_major_triad());
    assert!(minor_triad.is_minor_triad());
    assert!(!minor_triad.is_diminished_triad());
    assert!(!minor_triad.is_augmented_triad());

    // Test diminished triad
    let diminished_triad = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("Gb")]);
    assert!(!diminished_triad.is_major_triad());
    assert!(!diminished_triad.is_minor_triad());
    assert!(diminished_triad.is_diminished_triad());
    assert!(!diminished_triad.is_augmented_triad());

    // Test augmented triad
    let augmented_triad = PitchClassSet::new(&[note!("C"), note!("E"), note!("G#")]);
    assert!(!augmented_triad.is_major_triad());
    assert!(!augmented_triad.is_minor_triad());
    assert!(!augmented_triad.is_diminished_triad());
    assert!(augmented_triad.is_augmented_triad());

    // Test dominant seventh
    let dominant_seventh = PitchClassSet::new(&[note!("C"), note!("E"), note!("G"), note!("Bb")]);
    assert!(dominant_seventh.is_dominant_seventh());
    assert!(!dominant_seventh.is_major_seventh());
    assert!(!dominant_seventh.is_minor_seventh());
    assert!(!dominant_seventh.is_half_diminished_seventh());
    assert!(!dominant_seventh.is_fully_diminished_seventh());

    // Test major seventh
    let major_seventh = PitchClassSet::new(&[note!("C"), note!("E"), note!("G"), note!("B")]);
    assert!(!major_seventh.is_dominant_seventh());
    assert!(major_seventh.is_major_seventh());
    assert!(!major_seventh.is_minor_seventh());
    assert!(!major_seventh.is_half_diminished_seventh());
    assert!(!major_seventh.is_fully_diminished_seventh());

    // Test minor seventh
    let minor_seventh = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("G"), note!("Bb")]);
    assert!(!minor_seventh.is_dominant_seventh());
    assert!(!minor_seventh.is_major_seventh());
    assert!(minor_seventh.is_minor_seventh());
    assert!(!minor_seventh.is_half_diminished_seventh());
    assert!(!minor_seventh.is_fully_diminished_seventh());

    // Test half-diminished seventh
    let half_diminished_seventh = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("Gb"), note!("Bb")]);
    assert!(!half_diminished_seventh.is_dominant_seventh());
    assert!(!half_diminished_seventh.is_major_seventh());
    assert!(!half_diminished_seventh.is_minor_seventh());
    assert!(half_diminished_seventh.is_half_diminished_seventh());
    assert!(!half_diminished_seventh.is_fully_diminished_seventh());

    // Test fully diminished seventh
    let fully_diminished_seventh = PitchClassSet::new(&[note!("C"), note!("Eb"), note!("Gb"), note!("A")]);
    assert!(!fully_diminished_seventh.is_dominant_seventh());
    assert!(!fully_diminished_seventh.is_major_seventh());
    assert!(!fully_diminished_seventh.is_minor_seventh());
    assert!(!fully_diminished_seventh.is_half_diminished_seventh());
    assert!(fully_diminished_seventh.is_fully_diminished_seventh());

    // Test non-chord sets
    let non_chord = PitchClassSet::new(&[note!("C"), note!("D"), note!("E")]);
    assert!(!non_chord.is_major_triad());
    assert!(!non_chord.is_minor_triad());
    assert!(!non_chord.is_diminished_triad());
    assert!(!non_chord.is_augmented_triad());
}

#[test]
fn test_pitch_class_set_display() {
    let pc_set = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    let display = format!("{}", pc_set);

    // Should be sorted and formatted as a set
    assert!(display.starts_with("{"));
    assert!(display.ends_with("}"));
    assert!(display.contains("C"));
    assert!(display.contains("E"));
    assert!(display.contains("G"));

    let empty_set = PitchClassSet::new(&[]);
    assert_eq!(format!("{}", empty_set), "{}");
}

#[test]
fn test_pitch_class_set_from_conversions() {
    // Test From<Vec<NoteName>>
    let notes = vec![note!("C"), note!("E"), note!("G")];
    let pc_set: PitchClassSet = notes.into();
    assert_eq!(pc_set.len(), 3);
    assert!(pc_set.contains(note!("C")));
    assert!(pc_set.contains(note!("E")));
    assert!(pc_set.contains(note!("G")));

    // Test From<Vec<Pitch>>
    let pitches = vec![pitch!("C4"), pitch!("E4"), pitch!("G4")];
    let pc_set: PitchClassSet = pitches.into();
    assert_eq!(pc_set.len(), 3);
    assert!(pc_set.contains(note!("C")));
    assert!(pc_set.contains(note!("E")));
    assert!(pc_set.contains(note!("G")));
}

#[test]
fn test_pitch_class_set_enharmonic_equivalence() {
    // Test that enharmonic equivalents produce the same pitch class sets
    let set1 = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    let set2 = PitchClassSet::new(&[note!("C"), note!("Fb"), note!("G")]); // Fb = E
    let set3 = PitchClassSet::new(&[note!("B#"), note!("E"), note!("G")]); // B# = C

    // They should have the same prime form
    assert_eq!(set1.prime_form(), set2.prime_form());
    assert_eq!(set1.prime_form(), set3.prime_form());

    // They should have the same interval vector
    assert_eq!(set1.interval_vector(), set2.interval_vector());
    assert_eq!(set1.interval_vector(), set3.interval_vector());

    // They should have the same normal form (though note names may differ)
    assert_eq!(set1.normal_form().len(), set2.normal_form().len());
    assert_eq!(set1.normal_form().len(), set3.normal_form().len());
}

#[test]
fn test_pitch_class_set_duplicate_notes() {
    // Test that duplicate notes are handled correctly
    let notes = vec![note!("C"), note!("E"), note!("G"), note!("C"), note!("E")];
    let pc_set = PitchClassSet::new(&notes);

    // Should only contain unique pitch classes
    assert_eq!(pc_set.len(), 3);
    assert!(pc_set.contains(note!("C")));
    assert!(pc_set.contains(note!("E")));
    assert!(pc_set.contains(note!("G")));

    // Test with pitches in different octaves
    let pitches = vec![pitch!("C4"), pitch!("E4"), pitch!("G4"), pitch!("C5"), pitch!("E5")];
    let pc_set = PitchClassSet::from_pitches(&pitches);

    // Should still only contain unique pitch classes
    assert_eq!(pc_set.len(), 3);
    assert!(pc_set.contains(note!("C")));
    assert!(pc_set.contains(note!("E")));
    assert!(pc_set.contains(note!("G")));
}