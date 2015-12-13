static ES_EP_EB_EL_EY_IB_IL_IN_IE_EI_ER: &'static [&'static str] = &["ES", "EP", "EB", "EL", "EY", "IB", "IL", "IN", "IE", "EI", "ER"];
static L_T_K_S_N_M_B_Z: &'static [&'static str] = &["L", "T", "K", "S", "N", "M", "B", "Z"];

use phonetics::utils::metaphone_utils::*;
use phonetics::metaphone::conditions::{condition_c0, condition_ch0, condition_ch1, condition_m0, condition_l0};
use phonetics::metaphone::double_metaphone_result::DoubleMetaphoneResult;

pub fn handle_b(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    result.append('P');
    match char_at(&value, index + 1) {
        Some(ch) => if ch == 'B' {
            index + 2
        } else {
            index + 1
        },
        None => index + 1
    }
}

pub fn handle_c_cedilla(result: &mut DoubleMetaphoneResult, index: usize) -> usize {
    result.append('S');
    index + 1
}

pub fn handle_c(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if condition_c0(&value, index) {
        result.append('K');
        index + 2
    } else if index == 0 && contains(&value, index, 6, "CAESAR") {
        result.append('S');
        index + 2
    } else if contains(&value, index, 2, "CH") {
        handle_ch(result, &value, index)
    } else if contains(&value, index, 2, "CZ") && !(index > 1 && contains(&value, index - 2, 4, "WICZ")) {

        //-- "Czerny" --//
        result.append_primary('S');
        result.append_alternate('X');
        index + 2
    } else if contains(&value, index + 1, 3, "CIA") {
        //-- "focaccia" --//
        result.append('X');
        index + 3
    } else if contains(&value, index, 2, "CC") && ! (index == 1 && char_at_match(&value, 0, 'M')) {
        //-- double "cc" but not "McClelland" --//
        handle_cc(result, &value, index)
    } else if contains(&value, index, 2, "CK") || contains(&value, index, 2, "CG") || contains(&value, index, 2, "CQ") {
        result.append('K');
        index + 2
    } else if contains(&value, index, 2, "CI") || contains(&value, index, 2, "CE") || contains(&value, index, 2, "CY") {
        //-- Italian vs. English --//
        if contains(&value, index, 3, "CIO") || contains(&value, index, 3, "CIE") || contains(&value, index, 3, "CIA") {
            result.append_primary('S');
            result.append_alternate('X');
            index + 2
        } else {
            result.append('S');
            index + 2
        }
    } else {
        result.append('K');
        if contains(&value, index + 1, 2, " C") || contains(&value, index + 1, 2, " Q") || contains(&value, index + 1, 2, " G") {
            //-- Mac Caffrey, Mac Gregor --//
            index + 3
        } else if
            (contains(&value, index + 1, 1, "C") || contains(&value, index + 1, 1, "K") || contains(&value, index + 1, 1, "Q")) &&
                ! (contains(&value, index + 1, 2, "CE") || contains(&value, index + 1, 2, "CI")) {
            index + 2
        } else {
            index + 1
        }
    }
}

fn handle_cc(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if contains_at_least_one(&value, index + 2, 1, &["I", "E", "H"]) && !contains(&value, index + 2, 2, "HU") {
        //-- "bellocchio" but not "bacchus" --//
        if (index == 1 && char_at_match_before(&value, index, 1, 'A')) || (contains_at_least_one(&value, index - 1, 5, &["UCCEE", "UCCES"])) {
            //-- "accident", "accede", "succeed" --//
            result.append('K');
            result.append('S');
        } else {
            //-- "bacci", "bertucci", other Italian --//
            result.append('X');
        }
        index + 3
    } else {
        result.append('K');
        index + 2
    }
}

pub fn handle_ch(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if index > 0 && contains(&value, index, 4, "CHAE") {
        result.append_primary('K');
        result.append_alternate('X');
        index + 2
    } else if condition_ch0(&value, index) {
        //-- Greek roots ("chemistry", "chorus", etc.) --//
        result.append('K');
        index + 2
    } else if condition_ch1(&value, index) {
        //-- Germanic, Greek, or otherwise 'ch' for 'kh' sound --//
        result.append('K');
        index + 2
    } else {
        if index > 0 {
            if contains(&value, 0, 2, "MC") {
                result.append('K');
            } else {
                result.append_primary('X');
                result.append_alternate('K');
            }
        } else {
            result.append('X');
        }

        index + 2
    }
}

pub fn handle_d(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if contains(&value, index, 2, "DG") {
        //-- "Edge" --//
        if contains_at_least_one(&value, index + 2, 1, &["I", "E", "Y"]) {
            //-- "Edgar" --//
            result.append('J');
            index + 3
        } else {
            result.append('T');
            result.append('K');
            index + 2
        }
    } else if contains_at_least_one(&value, index, 2, &["DT", "DD"]) {
        result.append('T');
        index + 2
    } else {
        result.append('T');
        index + 1
    }
}

pub fn handle_f(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    double_next_char(result, value, index, 'F')
}

pub fn handle_g(result: &mut DoubleMetaphoneResult, value: &String, index: usize, slavo_germanic: bool) -> usize {
    let current_char = match char_at(&value, index + 1) {
        Some(v) => v,
        None => '_'
    };

    if current_char == 'H' {
        handle_gh(result, value, index)
    } else if current_char == 'N' {
        if index == 1 && is_vowel(&value, 0) && !slavo_germanic {
            result.append_primary('K');
            result.append_primary('N');
            result.append_alternate('N');
        } else if !contains(&value, index + 2, 2, "EY") && !slavo_germanic {
            result.append_primary('N');
            result.append_alternate('K');
            result.append_alternate('N');
        } else {
            result.append('K');
            result.append('N');
        }

        index + 2
    } else if contains(&value, index + 1, 2, "LI") && !slavo_germanic {
        result.append_primary('K');
        result.append_primary('L');
        result.append_alternate('L');
        index + 2
    } else if index == 0 && (current_char == 'Y' || contains_at_least_one(&value, index + 1, 2, ES_EP_EB_EL_EY_IB_IL_IN_IE_EI_ER)) {
        //-- -ges-, -gep-, -gel-, -gie- at beginning --//
        result.append_primary('K');
        result.append_alternate('J');
        index + 2
    } else if(contains(&value, index + 1, 2, "ER") || current_char == 'Y') &&
        !contains_at_least_one(&value, 0, 6, &["DANGER", "RANGER", "MANGER"]) &&
        !contains_at_least_one(&value, index - 1, 1, &["E", "I"]) &&
        !contains_at_least_one(&value, index - 1, 3, &["RGY", "OGY"]) {
            //-- -ger-, -gy- --//
            result.append_primary('K');
            result.append_alternate('J');
            index + 2
    } else if contains_at_least_one(&value, index + 1, 1, &["E", "I", "Y"]) || (index > 0 && contains_at_least_one(&value, index -1, 4, &["AGGI", "OGGI"])) {
        //-- Italian "biaggi" --//
        if contains_at_least_one(&value, 0, 4, &["VAN ", "VON "]) || contains(&value, 0, 3, "SCH") || contains(&value, index + 1, 2, "ET") {
            //-- obvious germanic --//
            result.append('K');
        } else if contains(&value, index + 1, 3, "IER") {
            result.append('J');
        } else {
            result.append_primary('J');
            result.append_alternate('K');
        }

        index + 2
    } else if current_char == 'G' {
        result.append('K');

        index + 2
    } else {
        result.append('K');
        index + 1
    }
}

fn handle_gh(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if index > 0 && ! is_vowel(&value, index - 1) {
        result.append('K');
        index + 2
    } else if index == 0 {
        let char_plus_two = match char_at(&value, index + 2) {
            Some(v) => v,
            None => '_'
        };

        if char_plus_two == 'I' {
            result.append('J');
        } else {
            result.append('K');
        }

        index + 2
    } else if (index > 1 && contains_at_least_one(&value, index - 2, 1, &["B", "H", "D"])) ||
        (index > 2 && contains_at_least_one(&value, index - 3, 1, &["B", "H", "D"])) ||
        (index > 3 && contains_at_least_one(&value, index - 4, 1, &["B", "H", "D"])) {
            //-- Parker's rule (with some further refinements) - "hugh"
            index + 2
        } else {
            if index > 2 && char_at_match_before(&value, index, 1, 'U') && contains_at_least_one(&value, index - 3, 1, &["C", "G", "L", "R", "T"]) {
                //-- "laugh", "McLaughlin", "cough", "gough", "rough", "tough"
                result.append('F');
            } else if index > 0 && !char_at_match_before(&value, index, 1, 'I') {
                result.append('K');
            }
            index + 2
        }
}

pub fn handle_h(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    //-- only keep if first & before vowel or between 2 vowels --//
    if (index == 0 || is_vowel(&value, index - 1)) && is_vowel(&value, index + 1) {
        result.append('H');
        index + 2
    } else {
        index + 1
    }
}

pub fn handle_j(result: &mut DoubleMetaphoneResult, value: &String, index: usize, slavo_germanic: bool) -> usize {
    if contains(&value, index, 4, "JOSE") || contains(&value, 0, 4, "SAN") {
        //-- obvious Spanish, "Jose", "San Jacinto" --//
        if  (index == 0 && (index > 4 && char_at_match_after(&value, index, 4, ' ')) || value.len() == 4) || contains(&value, 0, 4, "SAN ") {
            result.append('H');
        } else {
            result.append_primary('J');
            result.append_alternate('H');
        }

        index + 1
    } else {
        if index == 0 && !contains(&value, index, 4, "JOSE") {
            result.append_primary('J');
            result.append_alternate('A');
        } else if is_vowel(&value, index - 1) && !slavo_germanic && (char_at_match_after(&value, index, 1, 'A') || char_at_match_after(&value, index, 1, 'O')) {
            result.append_primary('J');
            result.append_alternate('H');
        } else if index == value.len() - 1 {
            result.append_primary('J');
            result.append_alternate(' ');
        } else if !contains_at_least_one(&value, index + 1, 1, L_T_K_S_N_M_B_Z) && !contains_at_least_one(&value, index - 1, 1, &["S", "K", "L"]) {
            result.append('J');
        }

        match char_at(&value, index + 1) {
            Some(v) if v == 'J' => index + 2,
            _ => index + 1
        }
    }
}

pub fn handle_k(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    result.append('K');
    match char_at(&value, index + 1) {
        Some(v) if v == 'K' => index + 2,
        _ => index + 1
    }
}

pub fn handle_l(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    match char_at(&value, index + 1) {
        Some(v) => {
            if v == 'L' {
                if condition_l0(&value, index) {
                    result.append_primary('L');
                } else {
                    result.append('L');
                }
                index + 2
            } else {
                result.append('L');
                index + 1
            }
        },
        None => {
            result.append('L');
            index + 1
        }
    }
}

pub fn handle_m(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    result.append('M');
    if condition_m0(&value, index) { index + 2 } else { index + 1 }
}

pub fn handle_n(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    double_next_char(result, value, index, 'N')
}

pub fn handle_p(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    let current_char = match char_at(&value, index + 1) {
        Some(v) => v,
        _ => '_'
    };
    if current_char == 'H' {
        result.append('F');
        index + 2
    } else {
        result.append('P');
        if contains_at_least_one(&value, index + 1, 1, &["P", "B"]) {
            index + 2
        } else {
            index + 1
        }
    }
}

pub fn handle_q(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    result.append('K');
    match char_at(&value, index + 1) {
        Some(v) if v == 'Q' => index + 2,
        _ => index + 1
    }
}

pub fn handle_r(result: &mut DoubleMetaphoneResult, value: &String, index: usize, slavo_germanic: bool) -> usize {
    if index == value.len() - 1 && !slavo_germanic && contains(&value, index - 2, 2, "IE") && !contains_at_least_one(&value, index - 4, 2, &["ME", "MA"]) {
        result.append_alternate('R');
    } else {
        result.append('R');
    }
    match char_at(&value, index + 1) {
        Some(v) if v == 'R' => index + 2,
        _ => index + 1
    }
}

pub fn handle_s(result: &mut DoubleMetaphoneResult, value: &String, index: usize, slavo_germanic: bool) -> usize {
    if index > 0 && contains_at_least_one(&value, index - 1, 3, &["ISL", "YSL"]) {
        //-- special cases "island", "isle", "carlisle", "carlysle" --//
        index + 1
    } else if index == 0 && contains(&value, index, 5, "SUGAR") {
        //-- special case "sugar" --//
        result.append_primary('X');
        result.append_alternate('S');
        index + 1
    } else if contains(&value, index, 2, "SH") {
        if contains_at_least_one(&value, index + 1, 4, &["HEIM", "HOEK", "HOLM", "HOLZ"]) {
            //-- germanic --//
            result.append('S');
        } else {
            result.append('X');
        }

        index + 2
    } else if contains_at_least_one(&value, index, 3, &["SIO", "SIA"]) || contains(&value, index, 4, "SIAN") {
        if slavo_germanic {
            result.append('S');
        } else {
            result.append_primary('S');
            result.append_alternate('X');
        }
        index + 3
    } else if (index == 0 && contains_at_least_one(&value, index + 1, 1, &["M", "N", "L", "W"])) || contains(&value, index + 1, 1, "Z") {
        //-- german & anglicisations, e.g. "smith" match "schmidt" //
        // "snider" match "schneider" --//
        //-- also, -sz- in slavic language although in hungarian it //
        //   is pronounced "s" --//
        result.append_primary('S');
        result.append_alternate('X');
        if contains(&value, index + 1, 1, "Z") {
            index + 2
        } else {
            index + 1
        }
    } else if contains(&value, index, 2, "SC") {
        handle_sc(result, &value, index)
    } else {
        if index == value.len() - 1 && contains_at_least_one(&value, index - 2, 2, &["AI", "OI"]) {
            //-- french e.g. "resnais", "artois" --//
            result.append_alternate('S');
        } else {
            result.append('S');
        }

        if contains_at_least_one(&value, index + 1, 1, &["S", "Z"]) {
            index + 2
        } else {
            index + 1
        }
    }
}

fn handle_sc(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    let current_char = match char_at(&value, index + 2) {
        Some(v) => v,
        _ => '_'
    };

    if current_char == 'H' {
        if contains_at_least_one(&value, index + 3, 2, &["OO", "ER", "EN", "UY", "ED", "EM"]) {
            //-- Dutch origin, e.g. "school", "schooner" --//
            if contains_at_least_one(&value, index + 3, 2, &["ER", "EN"]) {
                //-- "schermhorn", "shenker" --//
                result.append_primary('X');
                result.append_alternate('S');
                result.append_alternate('K');
            } else {
                result.append('S');
                result.append('K');
            }
        } else {
            if index == 0 && !is_vowel(&value, 3) && !char_at_match(&value, 3, 'W') {
                result.append_primary('X');
                result.append_alternate('S');
            } else {
                result.append('X');
            }
        }
    } else if contains_at_least_one(&value, index + 2, 1, &["I", "E", "Y"]) {
        result.append('S');
    } else {
        result.append('S');
        result.append('K');
    }

    index + 3
}

pub fn handle_t(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if contains(&value, index, 4, "TION") {
        result.append('X');
        index + 3
    } else if contains_at_least_one(&value, index, 3, &["TIA", "TCH"]) {
        result.append('X');
        index + 3
    } else if contains(&value, index, 2, "TH") || contains(&value, index, 3, "TTH") {
        if contains_at_least_one(&value, index + 2, 2, &["OM", "AM"]) ||
            contains_at_least_one(&value, 0, 4, &["VAN ", "VON "]) ||
            contains(&value, 0, 3, "SCH")
        {
            result.append('T');
        } else {
            result.append_primary('0');
            result.append_alternate('T');
        }
        index + 2
    } else {
        result.append('T');

        if contains_at_least_one(&value, index + 1, 1, &["T", "D"]) { index + 2 } else { index + 1 }
    }
}


pub fn handle_v(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    result.append('F');
    match char_at(&value, index + 1) {
        Some(v) if v == 'V' => index + 2,
        _ => index + 1
    }
}

pub fn handle_w(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if contains(&value, index, 2, "WR") {
        //-- can also be in middle of word --//
        result.append('R');
        index + 2
    } else {
        if index == 0 && (is_vowel(&value, index +1) || contains(&value, index, 2, "WH")) {
            if is_vowel(&value, index + 1) {
                //-- Wasserman should match Vasserman --//
                result.append_primary('A');
                result.append_alternate('F');
            } else {
                //-- need Uomo to match Womo --//
                result.append('A');
            }
            index + 1
        } else if (index > 0 && index == value.len()-1 && is_vowel(&value, index - 1)) || (index > 0 && contains_at_least_one(&value, index - 1, 5, &["EWSKI", "EWSKY", "OWSKI", "OWSKY"])) || contains(&value, 0, 3, "SCH") {
            //-- Arnow should match Arnoff --//
            result.append_alternate('F');
            index + 1
        } else if contains_at_least_one(&value, index, 4, &["WICZ", "WITZ"]) {
            //-- Polish e.g. "filipowicz" --//
            result.append_primary('T');
            result.append_primary('S');
            result.append_alternate('F');
            result.append_alternate('X');
            index + 4
        } else {
            index + 1
        }
    }
}

pub fn handle_x(result: &mut DoubleMetaphoneResult, value: &String, index: usize) -> usize {
    if index == 0 {
        result.append('S');
        index + 1
    } else {
        let index_test = index == value.len() - 1;
        let contains_iau_eau = index > 2 && contains_at_least_one(&value, index - 3, 3, &["IAU", "EAU"]);
        let contains_au_ou = contains_at_least_one(&value, index - 2, 2, &["AU", "OU"]);
        if !(index_test && (contains_iau_eau || contains_au_ou)) {
            //-- French e.g. breaux --//
            result.append('K');
            result.append('S');
        }

        if contains_at_least_one(&value, index + 1, 1, &["C", "X"]) { index + 2 } else { index + 1 }
    }
}

pub fn handle_z(result: &mut DoubleMetaphoneResult, value: &String, index: usize, slavo_germanic: bool) -> usize {
    let current_char = match char_at(&value, index + 1) {
        Some(v) => v,
        _ => '_'
    };
    if current_char == 'H' {
        //-- Chinese pinyin e.g. "zhao" or Angelina "Zhang" --//
        result.append('J');
        index + 2
    } else {
        if contains_at_least_one(&value, index + 1, 2, &["ZO", "ZI", "ZA"]) || (slavo_germanic && index > 0 && !char_at_match_before(&value, index, 1, 'T')) {
            result.append_primary('S');
            result.append_alternate('T');
            result.append_alternate('S');
        } else {
            result.append('S');
        }

        match char_at(&value, index + 1) {
            Some(v) if v == 'Z' => index + 2,
            _ => index + 1
        }
    }
}

fn double_next_char(result: &mut DoubleMetaphoneResult, value: &String, index: usize, letter: char) -> usize {
    result.append(letter);
    match char_at(&value, index + 1) {
        Some(v) => if v == letter { index + 2 } else { index + 1 },
        None => index + 1
    }
}
