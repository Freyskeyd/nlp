use std::cmp::Ordering;
use std::cmp::{min, max};
use std::iter::repeat;

fn max_length(a: &str, b: &str) -> usize {
    max(a.len(), b.len())
}

fn min_bound(i: &usize, search_range: &usize) -> usize {
    if i > search_range {
        max(0, i - search_range)
    } else {
        0
    }
}

fn max_bound(b: &str, i: &usize, search_range: &usize) -> usize {
    min(b.len() - 1, i + search_range)
}

/// Calculates the Jaro similarity between two strings. The returned value is between 0.0 and 1.0
/// (higher value means more similar).
///
/// Examples:
///
/// ```
/// use nlp::distance::jaro;
///
/// assert!((0.411 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() < 0.001);
/// ```
pub fn jaro(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }
    if a.len() == 0 || b.len() == 0 {
        return 0.0;
    }

    let a = a.to_uppercase();
    let b = b.to_uppercase();

    let search_range = max(0, (max_length(&a, &b) / 2) - 1);

    let mut b_consumed: Vec<bool> = repeat(false).take(b.len()).collect::<Vec<bool>>();

    let mut matches = 0.0;
    let mut transpositions = 0.0;
    let mut b_match_index = 0;

    for (i, a_char) in a.chars().enumerate() {
        let min_bound = min_bound(&i, &search_range);
        let max_bound = max_bound(&b, &i, &search_range);

        if min_bound > max_bound {
            continue;
        }

        for (j, b_char) in b.chars().enumerate() {
            if (min_bound <= j && j <= max_bound) && (a_char == b_char && !b_consumed[j]) {
                b_consumed[j] = true;
                matches += 1.0;

                if j < b_match_index {
                    transpositions += 1.0;
                }

                b_match_index = j;
                break;
            }
        }
    }

    if matches == 0.0 {
        0.0
    } else {
        (1.0 / 3.0) *
            ((matches / a.len() as f64) + (matches / b.len() as f64) +
             ((matches - transpositions) / matches))
    }
}

/// Like Jaro but gives a boost to strings that have a common prefix.
///
/// Examples:
///
/// ```
/// use nlp::distance::jaro_winkler;
///
/// assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
/// ```
pub fn jaro_winkler<T:ToString + ?Sized>(a: &T, b: &T) -> f64 {
    let a = a.to_string();
    let b = b.to_string();
    let jaro_distance = jaro(&a, &b);

    let prefrix_length = a.chars()
        .zip(b.chars())
        .take_while(|&(a_char, b_char)| a_char == b_char)
        .count();

    let jaro_winkler_distance = jaro_distance +
        (0.1 * prefrix_length as f64 * (1.0 - jaro_distance));

    if jaro_winkler_distance <= 1.0 {
        jaro_winkler_distance
    } else {
        1.0
    }
}

/// Calculates the minimum number of insertions, deletions and substitutions
/// required to change on string into the other.
///
/// Examples:
///
/// ```
/// use nlp::distance::levenshtein;
///
/// assert_eq!(3, levenshtein("kitten", "sitting"));
/// assert_eq!(2, levenshtein("pute", "puit"));
/// assert_eq!(0.5, 1.0 - ((levenshtein("puit", "pute") as f32) / ("puit".len() as f32)));
///
/// ```
pub fn levenshtein(a: &str, b: &str) -> usize {
    match a.cmp(&b) {
        Ordering::Equal => 0,
        _ => match a.len() {
            0 => b.len(),
            _ => match b.len() {
                0 => a.len(),
                _ => {
                    let mut prev_distances = (0..(b.len() as u16 + 1)).collect::<Vec<u16>>();
                    let mut curr_distances = repeat(0).take(b.len()+1).collect::<Vec<u16>>();

                    for (i, a_char) in a.chars().enumerate() {
                        curr_distances[0] = i as u16 + 1;

                        for (j, b_char) in b.chars().enumerate() {
                            curr_distances[j + 1]  = min(curr_distances[j] + 1,
                                                         min(prev_distances[j + 1] + 1, prev_distances[j] + (a_char != b_char) as u16));
                        }

                        prev_distances.clone_from(&curr_distances);
                    }

                    curr_distances[b.len()] as usize
                }
            }
        }
    }
}

/// Calculates the levenshtein distance between a string and each string in a vector. Returns a
/// vector of corresponding values.
///
/// Examples:
///
/// ```
/// use nlp::distance::levenshtein_against_vec;
///
/// let v = vec!["test", "test1", "test12", "test123", "", "tset"];
/// let result = levenshtein_against_vec("test", &v);
/// let expect = vec![0, 1, 2, 3, 4, 2];
/// assert_eq!(expect, result);
/// ```
pub fn levenshtein_against_vec(a: &str, v: &[&str]) -> Vec<usize> {
    let mut r: Vec<usize> = Vec::with_capacity(v.len());
    for b in v.iter() {
        r.push(levenshtein(a, b));
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_empty_string() {
        assert_eq!(0, levenshtein("", ""));
        let test: String = "hey".to_owned();
        assert_eq!(0, levenshtein(&test, &test))
    }

    #[test]
    fn levenshtein_same_string() {
        assert_eq!(0, levenshtein("kitten", "kitten"))
    }

    #[test]
    fn levenshtein_diff_short_string() {
        assert_eq!(3, levenshtein("kitten", "sitting"))
    }

    #[test]
    fn levenshtein_diff_spaced_sring() {
        assert_eq!(5, levenshtein("hello, world", "bye, world"))
    }

    #[test]
    fn levenshtein_diff_long_string() {
        let a = "The quick brown fox jumped over the angry dog.";
        let b = "Lorem ipsum dolor sit amet, dicta latine an eam.";
        assert_eq!(37, levenshtein(a, b))
    }

    #[test]
    fn levenshtein_first_empty() {
        assert_eq!(7, levenshtein("", "sitting"))
    }

    #[test]
    fn levenshtein_second_empty() {
        assert_eq!(6, levenshtein("kitten", ""))
    }

    // Jaro
    #[test]
    fn jaro_empty_string() {
        assert!((1.0 - jaro("", "")).abs() < 0.001)
    }

    #[test]
    fn jaro_first_empty() {
        assert!((0.0 - jaro("", "jaro")).abs() < 0.001)
    }

    #[test]
    fn jaro_second_empty() {
        assert!((0.0 - jaro("distance", "")).abs() < 0.001)
    }

    #[test]
    fn jaro_same() {
        assert!((1.0 - jaro("jaro", "jaro")).abs() < 0.001);
    }

    #[test]
    fn jaro_2() {
        assert!( (100.0 * jaro("FUCK", "FUKC").abs()) > 90.0 );
        assert!( (100.0 * jaro("Fuck", "FUKC").abs()) > 90.0 );
    }

    #[test]
    fn jaro_diff_short() {
        assert!((0.767 - jaro("dixon", "dicksonx")).abs() < 0.001)
    }

    #[test]
    fn jaro_diff_no_transposition() {
        assert!((0.944 - jaro("martha", "marhta")).abs() < 0.001)
    }

    #[test]
    fn jaro_diff_with_transposition() {
        assert!((0.411 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() < 0.001)
    }

    #[test]
    fn jaro_1() {
        assert!((0.411 - jaro(&"Friedrich Nietzsche".to_owned(), &"Jean-Paul Sartre".to_owned())).abs() < 0.001)
    }

    #[test]
    fn levenshtein_only_strings() {
        let vec: Vec<String> = vec!["test".to_owned(), "bibi".to_owned()];

        let mut vv: Vec<&str> = Vec::new();
        for v in &vec {
            vv.push(&v);
        }
        let test = "test".to_owned();
        assert_eq!(levenshtein_against_vec(&test, &vv[..]), [0, 4])
    }
}
