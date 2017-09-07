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

#[bench]
fn bench_jaro(b: &mut Bencher) {
    b.iter(|| jaro("rubert", "rupert"));
}

#[bench]
fn bench_jaro_winkler(b: &mut Bencher) {
    b.iter(|| jaro_winkler("rubert", "rupert"));
}

#[bench]
#[ignore]
fn bench_levenshtein_fast_10_k(b: &mut Bencher) {

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

#[bench]
#[ignore]
fn bench_jaro_fast_10_k(b: &mut Bencher) {

    let path = Path::new("words2.txt");
    let mut s = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut s).unwrap();

    let wbyl: Vec<String> = s.lines().map(|s| s.to_owned()).collect();
    b.iter(|| {
        for i in &wbyl {
            jaro("rubert", &i);
        }
    })
}

#[bench]
#[ignore]
fn bench_jaro_winkler_fast_10_k(b: &mut Bencher) {

    let path = Path::new("words2.txt");
    let mut s = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut s).unwrap();

    let wbyl: Vec<String> = s.lines().map(|s| s.to_owned()).collect();
    b.iter(|| {
        for i in &wbyl {
            jaro_winkler("rubert", &i);
        }
    })
}
