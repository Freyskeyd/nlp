use std::fmt::Display;
static FRONTV: &'static str = "EIY";
static VOWELS: &'static str = "AEIOU";
static VARSON: &'static str = "CSPTG";
const MAX_LEN: usize = 10;

/// Try metaphone
///
/// Examples:
///
/// ```
/// use nlp::phonetics::metaphone::metaphone;
/// assert_eq!("SMN", metaphone("simon"));
/// assert_eq!("S", metaphone("s"));
/// ```
pub fn metaphone<T:ToString + ?Sized>(word: &T) -> String {
    let word = word.to_string();
    if word.is_empty() {
        return "".to_owned();
    }

    if word.len() == 1 {
        return word.to_uppercase().to_owned();
    }

    let upper_word = word.to_uppercase();
    let word_char = upper_word
        .chars()
        .collect::<Vec<_>>();

    // println!("{:?}", word_char);
    let mut local = String::new();

    let action_when_second_letter_match = |string: &mut String, letter: char| {
        string.push_str(&upper_word);
        if letter == word_char[1] {
            string.remove(0);
        }
    };

    match word_char[0] {
        'K' | 'G' | 'P'            => action_when_second_letter_match(&mut local, 'N'),
        'A'                        => action_when_second_letter_match(&mut local, 'E'),
        'W' if word_char[1] == 'R' => action_when_second_letter_match(&mut local, 'R'),
        'W' if word_char[1] == 'H' => {action_when_second_letter_match(&mut local, 'H'); local.remove(0); local.insert(0, 'W');},
        'W'                        => local.push_str(&upper_word),
        'X'                        => {local.push_str(&upper_word); local.remove(0); local.insert(0, 'S');},
        _                          => local.push_str(&upper_word)
    }

    let local_size = local.len();

    let mut code = String::new();

    let mut n = 0;
    while (code.len() < MAX_LEN) && n < local_size {
        let char_at = local.chars().clone().nth(n).unwrap_or('_');
        let char_next = local.chars().clone().nth(n + 1).unwrap_or('_');

        // println!("code_len: {}, n: {}, char_at: {}, char_next: {}", code.len(), n, char_at, char_next);
        if char_at != 'C' && is_previous_char(&local, n, char_at) {
            n += 1;
        } else {
            match char_at {
                'A' | 'E' | 'I' | 'O' | 'U' if n == 0                                                                                         => code.push(char_at),
                'B' if b_testing(&local, n)                                                                                                   => code.push(char_at),
                'C' if c_testing(&local, n, char_next) && region_match(&local, n, "CIA")                                                      => code.push('X'),
                'C' if c_testing(&local, n, char_next) && !is_last_char(local_size, n) && FRONTV.contains(char_next)                          => code.push('S'),
                'C' if c_testing(&local, n, char_next) && is_previous_char(&local, n, 'S') && is_next_char(&local, n, 'H')                    => code.push('K'),
                'C' if c_testing(&local, n, char_next) && is_next_char(&local, n, 'H') && (n == 0 && local_size >= 3 && is_vowel(&local, 2))  => code.push('K'),
                'C' if c_testing(&local, n, char_next) && is_next_char(&local, n, 'H') && !(n == 0 && local_size >= 3 && is_vowel(&local, 2)) => code.push('X'),
                'C' if c_testing(&local, n, char_next)                                                                                        => code.push('K'),
                'D' if d_testing(&local, n)                                                                                                   => {code.push('J'); n += 2;},
                'D'                                                                                                                           => code.push('T'),
                'G' if g_testing(&local, n) && g_testing_j(&local, n)                                                                         => code.push('J'),
                'G' if g_testing(&local, n)                                                                                                   => code.push('K'),
                'H' if h_testing(&local, n) && is_vowel(&local, n + 1)                                                                        => code.push('H'),
                'F' | 'J' | 'L' | 'M' | 'N' | 'R'                                                                                             => code.push(char_at),
                'K' if n == 0 || !is_previous_char(&local, n, 'C')                                                                            => code.push(char_at),
                'P' if is_next_char(&local, n, 'H')                                                                                           => code.push('F'),
                'P'                                                                                                                           => code.push(char_at),
                'Q'                                                                                                                           => code.push('K'),
                'S' if region_match(&local, n, "SH") || region_match(&local, n, "SIO") || region_match(&local, n, "SIA")                      => code.push('X'),
                'S'                                                                                                                           => code.push('S'),
                'T' if region_match(&local, n, "TIA") || region_match(&local, n, "TIO")                                                       => code.push('X'),
                'T' if !region_match(&local, n, "TCH") && region_match(&local, n, "TH")                                                       => code.push('0'),
                'T' if !region_match(&local, n, "TCH")                                                                                        => code.push('T'),
                'V'                                                                                                                           => code.push('F'),
                'W' | 'Y' if !is_last_char(local_size, n) && is_vowel(&local, n + 1)                                                          => code.push(char_at),
                'X'                                                                                                                           => {code.push('K'); code.push('S');},
                'Z'                                                                                                                           => code.push('S'),
                '_' => {
                    break;
                }
                _ => ()
            };
            n += 1;
        }
    }
    code
}

fn g_testing_j(local: &str, n: usize) -> bool {
    !is_last_char(local.len(), n) && FRONTV.contains(local.chars().nth(n + 1).unwrap()) && !is_previous_char(&local, n, 'G')
}

fn d_testing(local: &str, n: usize) -> bool {
    !is_last_char(local.len(), n + 1) && is_next_char(&local, n, 'G') && FRONTV.contains(local.chars().nth(n + 2).unwrap())
}

fn h_testing(local: &str, n: usize) -> bool {
    !(is_last_char(local.len(), n) || (n > 0 && VARSON.contains(local.chars().nth(n - 1).unwrap())))
}

fn g_testing(local: &str, n: usize) -> bool {
    let last_char = is_last_char(local.len(), n + 1);
    let next_is_h = is_next_char(local, n, 'H');

    (!(last_char && next_is_h) && !(!last_char && next_is_h && !is_vowel(local, n + 2)) && !(n > 0 && (region_match(local, n, "GN") || region_match(local, n, "GNED"))))
}

fn b_testing(local: &str, n: usize) -> bool {
    !(is_previous_char(&local, n, 'M') && is_last_char(local.len(), n))
}

fn c_testing(local: &str, n: usize, char_next: char) -> bool {
    ! (is_previous_char(&local, n , 'S') && !is_last_char(local.len(), n) && FRONTV.contains(char_next))
}

fn is_vowel(local: &str, n: usize) -> bool {
    VOWELS.contains(local.chars().nth(n).unwrap())
}

fn is_next_char(local: &str, n: usize, test: char) -> bool {
    if n < local.len() -1 {
        return local.chars().nth(n + 1).unwrap() == test;
    }

    false
}

fn region_match(local: &str, n: usize, test: &str) -> bool {
    if  (n + test.len() - 1) < local.len(){
        return local.chars().skip(n).take(test.len()).collect::<String>() == test;
    }

    false
}

fn is_previous_char(local: &str, n: usize, current: char) -> bool {
    if n > 0 && n < local.len() {
        return local.chars().clone().nth(n - 1).unwrap() == current;
    }

    false
}


fn is_last_char(size: usize, n: usize) -> bool {
    n + 1 == size
}

