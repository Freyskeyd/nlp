# nlp

[![Build Status](https://travis-ci.org/Freyskeyd/nlp.svg)](https://travis-ci.org/Freyskeyd/nlp) [![Coverage Status](https://coveralls.io/repos/Freyskeyd/nlp/badge.svg?service=github)](https://coveralls.io/github/Freyskeyd/nlp)

Rust-nlp

## Implemented algorithm

### Distance
- [x] Levenshtein
- [x] Jaro
- [x] Jaro winkler

### Phonetics
- [  ] Soundex
- [  ] Double-metaphone / metaphone
- [  ] Daitch-mokotoff
- [  ] NYSIIS

## Development workflow

Use multirust with `nightly`.

```bash
$ cd to/project
$ multirust override nightly
$ rustc -V
rustc 1.X.X-nightly
$ cargo test --features "dev"
$ multirust remove-override
$ rustc -V
rustc 1.X.X
$ cargo test
```
