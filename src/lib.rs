#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]


#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![doc="Natural language processing library"]


#[macro_use] extern crate itertools;

/// Distance module (Levenshtein, Jaro, Jaro-winkler)
pub mod distance;

/// Phonetics module (Soundex)
pub mod phonetics;
