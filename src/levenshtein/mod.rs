/**
 * Levenshtein Distance implementation
 * - distance(s1: &str, s2: &str)  => distance_unicode(...)
 * - distance_unicode(s1: &str, s2: &str) -> usize
 * - distance_byte(s1: &str, s2: &str) -> usize
 * 
 * TODO:
 * - Implement Interpretations::Words (tokenize on whitespace)
 * - Implement Interpretations::NGram(w) (tokenize on width)
 */

mod distance;

pub use distance::{distance, distance_unicode, distance_byte};