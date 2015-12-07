use phonetics::utils::metaphone_utils::char_at;
use phonetics::metaphone::handler::*;
use phonetics::metaphone::double_metaphone_result::DoubleMetaphoneResult;

static SILENT_START: &'static [&'static str] = &["GN", "KN", "PN", "WR", "PS"];
// static ES_EP_EB_EL_EY_IB_IL_IN_IE_EI_ER: &'static Vec<&str> = ["ES", "EP", "EB", "EL", "EY", "IB", "IL", "IN", "IE", "EI", "ER"];
// static L_T_K_S_N_M_B_Z: &'static Vec<&str> = ["L", "T", "K", "S", "N", "M", "B", "Z"];

const MAX_LEN: i32 = 4;

/// Double metaphone with alternate
pub fn double_metaphone(value: &str) -> Option<DoubleMetaphoneResult> {
    let value:String = match clean_input(&value) {
        Some(v) => v,
        None => return None
    };

    let slavo_germanic: bool = is_slavo_germanic(&value);
    let mut index: usize = if is_silent_start(&value) {
        1
    } else {
        0
    };

    let mut result = DoubleMetaphoneResult::new(MAX_LEN);

    while !result.is_complete() && index <= value.len() - 1 {

        let current_char = match char_at(&value, index) {
            Some(v) => v,
            None => {break;}
        };

        index = match current_char {
            'A'| 'E'| 'I'| 'O'| 'U'| 'Y' => handle_aeiouy(&mut result, index),
            'B'                          => handle_b(&mut result, &value, index),
            'Ç'                          => handle_c_cedilla(&mut result, index),
            'C'                          => handle_c(&mut result, &value, index),
            'D'                          => handle_d(&mut result, &value, index),
            'F'                          => handle_f(&mut result, &value, index),
            'G'                          => handle_g(&mut result, &value, index, slavo_germanic),
            'H'                          => handle_h(&mut result, &value, index),
            'J'                          => handle_j(&mut result, &value, index, slavo_germanic),
            'K'                          => handle_k(&mut result, &value, index),
            'L'                          => handle_l(&mut result, &value, index),
            'M'                          => handle_m(&mut result, &value, index),
            'N'                          => handle_n(&mut result, &value, index),
            'P'                          => handle_p(&mut result, &value, index),
            'Q'                          => handle_q(&mut result, &value, index),
            'R'                          => handle_r(&mut result, &value, index, slavo_germanic),
            'S'                          => handle_s(&mut result, &value, index, slavo_germanic),
            'T'                          => handle_t(&mut result, &value, index),
            'V'                          => handle_v(&mut result, &value, index),
            'W'                          => handle_w(&mut result, &value, index),
            'X'                          => handle_x(&mut result, &value, index),
            'Z'                          => handle_z(&mut result, &value, index, slavo_germanic),
            'Ñ'|_ => index + 1
        }
    }

    result.cleanup();

    Some(result)
}

// Handler
// Private methods
fn handle_aeiouy(result: &mut DoubleMetaphoneResult, index: usize) -> usize {
    if index == 0 {
        result.append('A');
    }

    index + 1
}

fn is_slavo_germanic(value: &str) -> bool {
    value.contains("W") || value.contains("K") || value.contains("CZ") || value.contains("WITZ")
}

fn is_silent_start(value: &str) -> bool {
    SILENT_START
        .iter()
        .map(|silent: &&str| value.starts_with(silent))
        .fold(false, |acc, x| acc || x)
}

fn clean_input(value: &str) -> Option<String> {
    if value.len() == 0 {
        return None;
    }

    let value = value.trim();

    if value.len() == 0 {
        return None;
    }

    Some(value.to_uppercase())
}
