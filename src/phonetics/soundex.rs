use itertools::Itertools;

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
/// Soundex char mapping
pub enum SoundexChar {
    /// B, F, P, V
    S1,
    /// C, G, J, K, Q, S, X, Z
    S2,
    /// D, T
    S3,
    /// L
    S4,
    /// M, N
    S5,
    /// R
    S6,
    /// Vowels
    Vowel,
    /// H, W
    HW,
    /// Space
    Space,
}

/// Encode a normal char into a soundex char
fn encode(ch: char) -> SoundexChar {
    if let Some(c) = ch.to_lowercase().next() {
        match c {
            'b' | 'f' | 'p' | 'v'                         => SoundexChar::S1,
            'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => SoundexChar::S2,
            'd' | 't'                                     => SoundexChar::S3,
            'l'                                           => SoundexChar::S4,
            'm' | 'n'                                     => SoundexChar::S5,
            'r'                                           => SoundexChar::S6,
            'h' | 'w'                                     => SoundexChar::HW,
            ' '                                           => SoundexChar::Space,
            _                                             => SoundexChar::Vowel,
        }
    } else {
        SoundexChar::Vowel
    }
}

#[derive(PartialEq, Hash, Clone, Debug)]
/// A soundex word
pub struct SoundexWord {
    first_char: char,
    soundex_chars: Vec<SoundexChar>,
}

impl SoundexWord {
    /// Generate a soundex word base on a string
    fn new(word: &str) -> SoundexWord {
        let first_char = word
            .chars().clone().nth(0)
            .unwrap_or('_');

        let soundex_chars = word
            .chars()
            .map(encode)
            .skip(1)
            .filter(|x| *x != SoundexChar::HW)
            .dedup()
            .filter(|x| *x != SoundexChar::Vowel)
            .collect::<Vec<_>>();

        SoundexWord {
            first_char: first_char,
            soundex_chars: soundex_chars
        }
    }

    /// Convert soundex word to the normal soundex representation
    fn to_string(&self) -> String {
        self.first_char.to_string() +
            &self.soundex_chars.iter().map(|x| {
                match *x {
                    SoundexChar::S1    => "1",
                    SoundexChar::S2    => "2",
                    SoundexChar::S3    => "3",
                    SoundexChar::S4    => "4",
                    SoundexChar::S5    => "5",
                    SoundexChar::S6    => "6",
                    SoundexChar::Space => "",
                    _                  => "_",
                }.to_owned()
            }).join("")
    }
}

/// Try soundex
///
/// Examples:
///
/// ```
/// use nlp::phonetics::soundex::soundex;
/// assert_eq!("S550", soundex("SIMON "));
/// ```
pub fn soundex(a: &str) -> String {
    let soundex_a = SoundexWord::new(a).to_string();

    let mut s = String::new();
    s.push_str(&soundex_a[..]);

    if s.len() < 4 {
        for _ in (0..(4 - s.len())).enumerate() {
            s.push('0');
        }
    }

    s.truncate(4);
    s.to_uppercase()
}

/// Compare soundex words
///
/// Examples:
///
/// ```
/// use nlp::phonetics::soundex::compare_soundex_words;
///
/// assert!(compare_soundex_words("SIMON", "SIMON"));
/// assert!(compare_soundex_words("hey", "hei"));
/// assert!(compare_soundex_words("america", "amurica"));
/// assert!(compare_soundex_words("lol", "lul"));
/// assert!(!compare_soundex_words("purée", "bisous"));
/// assert!(compare_soundex_words("purée", "purée"));
/// ```
pub fn compare_soundex_words(a: &str, b: &str) -> bool {
    soundex(a) == soundex(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soundex_1() {
        assert_eq!("S550", soundex("SIMON"))
    }

    #[test]
    fn soundex_2() {
        assert_eq!("S553", soundex("Simon TESTing"))
    }

    #[test]
    fn soundex_3() {
        assert_eq!("A261", soundex("Ashcroft"))
    }

    #[test]
    fn soundex_4() {
        assert_eq!("P600", soundex("purée"))
    }

    #[test]
    fn soundex_5() {
        assert_eq!("P300", soundex("putée"))
    }

    #[test]
    fn soundex_6() {
        assert!(compare_soundex_words("putée", "putée"))
    }

    // False positive
    #[test]
    fn soundex_7() {
        assert!(compare_soundex_words("hey rupert", "hei robert"))
    }

    // False positive
    #[test]
    fn soundex_8() {
        assert!(compare_soundex_words("what's up", "wats oops"))
    }

    // False positive
    #[test]
    fn soundex_9() {
        assert!(compare_soundex_words("lol your stupid", "lul ur stoupid"))
    }

}
