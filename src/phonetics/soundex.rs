/// Try soundex
///
/// Examples:
///
/// ```
/// use nlp::phonetics::soundex::soundex;
/// assert_eq!("SIMON", soundex("SIMON ", ""));
/// ```
pub fn soundex(a: &str, b: &str) -> String {
    a.trim().to_uppercase()
}
