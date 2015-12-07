static VOWELS: &'static str = "AEIOUY";
static VOWELS_NO_Y: &'static str = "AEIOU";

pub fn is_vowel(local: &str, n: usize) -> bool {
    match local.chars().nth(n) {
        Some(v) => VOWELS.contains(v),
        None => false
    }
}

pub fn is_vowel_without_y(local: &str, n: usize) -> bool {
    match local.chars().nth(n) {
        Some(v) => VOWELS_NO_Y.contains(v),
        None => false
    }
}

pub fn contains(value: &String, start: usize, len: usize, search: &str) -> bool {
    if  (start + len) <= value.len() {
        value.chars().skip(start).take(search.len()).collect::<String>() == search
    } else {
        false
    }
}

pub fn contains_at_least_one(value: &String, start: usize, len: usize, search: &[&str]) -> bool {
    search
        .iter()
        .map(|x| contains(value, start, len, x))
        .fold(false, |acc, x| acc || x)
}

pub fn char_at(value: &String, index: usize) -> Option<char> {
    value.chars().nth(index)
}

pub fn char_at_match(value: &String, index: usize, search: char) -> bool {
    match char_at(value, index) {
        Some(v) if v == search => true,
        _ => false
    }
}

pub fn char_at_match_before(value: &String, index: usize, reduce: usize, search: char) -> bool {
    if index == 0 && reduce > 0 {
        false
    } else {
        char_at_match(value, index - reduce, search)
    }
}

pub fn char_at_match_after(value: &String, index: usize, increase: usize, search: char) -> bool {
    if (index + increase) > value.len() - 1 {
        false
    } else {
        char_at_match(value, index + increase, search)
    }
}

#[test]
fn test_contains_at_least_one() {
    let test = "bisoustoussa".to_owned();
    assert!(contains_at_least_one(&test, 0, 3, &["bis", "tt"]));
}

