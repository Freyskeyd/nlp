#![feature(test)]

extern crate nlp;
use nlp::distance::*;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate test;

use test::Bencher;

#[bench]
fn bench_levenshtein(b: &mut Bencher) {
    b.iter(|| levenshtein("rubert", "rupert"));
}

// #[bench]
// fn bench_levenshtein_fast(b: &mut Bencher) {
//     b.iter(|| levenshtein_fast("rubert", "rupert"));
// }

#[bench]
fn bench_levenshtein_fast_10K(b: &mut Bencher) {

    let path = Path::new("words2.txt");
    let mut s = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut s).unwrap();

    let wbyl: Vec<String> = s.lines().map(|s| s.to_owned()).collect();
    b.iter(|| {
        for i in &wbyl {
            levenshtein("rubert", &i);
        }
    })
}
