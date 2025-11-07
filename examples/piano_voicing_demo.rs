//! Demo program showing piano voicing functionality

use chordy::{Chord, note};

fn main() {
    let chord = Chord::major(note!("C"));

    println!("Testing piano voicing methods for C major chord...");

    // Test that piano voicing methods are accessible
    if let Ok(block_voiced) = chord.voice_piano_block() {
        println!("✓ Block voicing works: {:?}", block_voiced.pitches);
    }

    if let Ok(spread_voiced) = chord.voice_piano_spread() {
        println!("✓ Spread voicing works: {:?}", spread_voiced.pitches);
    }

    if let Ok(jazz_voiced) = chord.voice_piano_jazz() {
        println!("✓ Jazz voicing works: {:?}", jazz_voiced.pitches);
    }

    if let Ok(rootless_voiced) = chord.voice_piano_rootless() {
        println!("✓ Rootless voicing works: {:?}", rootless_voiced.pitches);
    }

    if let Ok(broken_voiced) = chord.voice_piano_broken() {
        println!("✓ Broken voicing works: {:?}", broken_voiced.pitches);
    }

    println!("All piano voicing methods are accessible!");
}