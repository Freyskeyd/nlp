// Conditions
use phonetics::utils::metaphone_utils::{is_vowel, contains, contains_at_least_one, char_at, char_at_match_after};

static L_R_N_M_B_H_F_V_W_SPACE: &'static [&'static str] = &["L", "R", "N", "M", "B", "H", "F", "V", "W", "SPACE"];

pub fn condition_c0(value: &String, index: usize) -> bool {
    if contains(&value, index, 4, "CHIA") {
        true
    } else if index <= 1 {
        false
    } else if is_vowel(&value, index - 2) ||
        !contains(&value, index -1, 3, "ACH") {
        false
    } else {
        (!char_at_match_after(&value, index, 2, 'I') && !char_at_match_after(&value, index, 2, 'E')) || contains_at_least_one(&value, index - 2, 6, &["BACHER", "MACHER"])
    }
}

pub fn condition_ch0(value: &String, index: usize) -> bool {
    if index != 0 {
        false
    } else if
        !contains_at_least_one(&value, index + 1, 5, &["HARAC", "HARIS"])  &&
        !contains_at_least_one(&value, index + 1, 3, &["HOR", "HYM", "HIA", "HEM"]) {
        false
    } else if contains(&value, 0, 5, "CHORE") {
        false
    } else {
        true
    }
}

pub fn condition_ch1(value: &String, index: usize) -> bool {
    let a = contains_at_least_one(&value, 0, 4, &["VAN", "VON"]);
    let b = index > 1 && contains_at_least_one(&value, index - 2, 6, &["ORCHES", "ARCHIT", "ORCHID"]);
    let c = contains_at_least_one(&value, index + 2, 1, &["T", "S"]);
    let d = index > 0 && contains_at_least_one(&value, index - 1, 1, &["A", "O", "U", "E"]) || index == 0;
    let e = contains_at_least_one(&value, index + 2, 1, L_R_N_M_B_H_F_V_W_SPACE) || index + 1 == value.len() - 1;

    a || b || c || (d && e)
}

pub fn condition_m0(value: &String, index: usize) -> bool {
    let current_char = match char_at(&value, index + 1) {
        Some(v) => v,
        _ => '_'
    };

    if current_char == 'M' {
        true
    } else {
        index > 0 && contains(&value, index - 1, 3, "UMB") && (index + 1 == value.len() -1 || contains(&value, index + 2, 2, "ER"))
    }
}

pub fn condition_l0(value: &String, index: usize) -> bool {
    if index == value.len() - 3 && contains_at_least_one(&value, index - 1, 4, &["ILLO", "ILLA", "ALLE"]) {
        true
    } else if (contains_at_least_one(&value, value.len() - 2, 2, &["AS", "OS"]) || contains_at_least_one(&value, value.len() - 1, 1, &["A", "O"])) && contains(&value, index - 1, 4, "ALLE") {
        true
    } else {
        false
    }
}
