// use core::str::CharEq;

// fn tokenize(text: &str) -> Vec<&str> {
//     text.split(Splitter).filter(|s| s.len() > 0).collect()
// }

// struct Splitter;

// impl CharEq for Splitter {
//     fn matches(&mut self, c: char) -> bool {
//         match c {
//             ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
//             | ':' | '\t' | '\n' | '(' | ')' | '-' => true,
//             _ => false
//         }
//     }

//     fn only_ascii(&self) -> bool {
//         true
//     }
// }
