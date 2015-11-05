use std::cmp::{min, max};

/// Calculates the Jaro similarity between two strings. The returned value is between 0.0 and 1.0
/// (higher value means more similar).
///
/// Examples:
/// ```
/// assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() < 0.001);
/// ```
pub fn jaro(a: &str, b: &str) -> f64 {
    if a == b { return 1.0; }
    if a.len() == 0 || b.len() == 0 { return 0.0; }

    let search_range = max(0, (max(a.len(), b.len()) / 2) - 1);

    let mut b_consumed = Vec::with_capacity(b.len());
    for _ in 0..b.len() {
        b_consumed.push(false);
    }

    let mut matches = 0.0;
    let mut transpositions = 0.0;
    let mut b_match_index = 0;

    for (i, a_char) in a.chars().enumerate() {
        let min_bound =
            // prevent integer wrapping
            if i > search_range {
                max(0, i - search_range)
            } else {
                0
            };

        let max_bound = min(b.len() - 1, i + search_range);

        if min_bound > max_bound {
            continue;
        }

        for (j, b_char) in b.chars().enumerate() {
            if min_bound <= j && j <= max_bound {
                if a_char == b_char && !b_consumed[j] {
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
    }

    if matches == 0.0 {
        0.0
    } else {
        (1.0 / 3.0) * ((matches / a.len() as f64) +
                       (matches / b.len() as f64) +
                       ((matches - transpositions) / matches))
    }
}

/// Like Jaro but gives a boost to strings that have a common prefix.
///
/// Examples:
/// ```
/// assert!((0.911 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
/// ```
pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    let jaro_distance = jaro(a, b);

    let prefrix_length = a.chars()
        .zip(b.chars())
        .take_while(|&(a_char, b_char)| a_char == b_char)
        .count();

    let jaro_winkler_distance = jaro_distance + (0.1 * prefrix_length as f64 * (1.0 - jaro_distance));

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
    if a == b { return 0; }
    else if a.len() == 0 { return b.len(); }
    else if b.len() == 0 { return a.len(); }

    let mut prev_distances: Vec<usize> = Vec::with_capacity(b.len() + 1);
    let mut curr_distances: Vec<usize> = Vec::with_capacity(b.len() + 1);

    for i in 0..(b.len() + 1) {
        prev_distances.push(i);
        curr_distances.push(0);
    }

    for (i, a_char) in a.chars().enumerate() {
        curr_distances[0] = i + 1;

        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            curr_distances[j + 1] = min(curr_distances[j] + 1,
                                        min(prev_distances[j + 1] +1,
                                            prev_distances[j] + cost));
        }

        prev_distances.clone_from(&curr_distances);
    }

    curr_distances[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_empty_string() {
        assert_eq!(0, levenshtein("", ""))
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
        assert_eq!(1.0, jaro("",""))
    }

    #[test]
    fn jaro_first_empty() {
        assert_eq!(0.0, jaro("", "jaro"))
    }

    #[test]
    fn jaro_second_empty() {
        assert_eq!(0.0, jaro("distance", ""))
    }

    #[test]
    fn jaro_same() {
        assert_eq!(1.0, jaro("jaro", "jaro"))
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
        assert!((0.392 - jaro("Friedrich Nietzsche",
                              "Jean-Paul Sartre")).abs() < 0.001)
    }
}
