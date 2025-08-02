#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    #[cfg(feature = "grapheme")]
    return input.graphemes(true).rev().collect();

    #[cfg(not(feature = "grapheme"))]
    return input.chars().rev().collect();
}
