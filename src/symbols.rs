#[cfg(feature = "utf8_symbols")]
pub const FLAT: &str = "♭";
#[cfg(not(feature = "utf8_symbols"))]
pub const FLAT: &str = "b";

#[cfg(feature = "utf8_symbols")]
pub const SHARP: &str = "♯";
#[cfg(not(feature = "utf8_symbols"))]
pub const SHARP: &str = "#";

#[cfg(feature = "utf8_symbols")]
pub const DOUBLE_FLAT: &str = "𝄫";
#[cfg(not(feature = "utf8_symbols"))]
pub const DOUBLE_FLAT: &str = "bb";

#[cfg(feature = "utf8_symbols")]
pub const DOUBLE_SHARP: &str = "𝄪";
#[cfg(not(feature = "utf8_symbols"))]
pub const DOUBLE_SHARP: &str = "##";

#[cfg(feature = "utf8_symbols")]
pub const NATURAL: &str = "♮";
#[cfg(not(feature = "utf8_symbols"))]
pub const NATURAL: &str = "♮"; // Still use it even without utf8 feature

// Note names
pub const C: &str = "C";
pub const D: &str = "D";
pub const E: &str = "E";
pub const F: &str = "F";
pub const G: &str = "G";
pub const A: &str = "A";
pub const B: &str = "B";
