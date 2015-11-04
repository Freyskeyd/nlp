use std::cmp::{min};

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
/// assert_eq!(50, (levenshtein("puit", "pute") / "puit".len()));
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
}
