# nlp

[![Build Status](https://travis-ci.org/Freyskeyd/nlp.svg)](https://travis-ci.org/Freyskeyd/nlp) [![Coverage Status](https://coveralls.io/repos/Freyskeyd/nlp/badge.svg?service=github)](https://coveralls.io/github/Freyskeyd/nlp)

Rust-nlp

## Implemented algorithm

### Distance
- [x] Levenshtein ([Explanation](https://fr.wikipedia.org/wiki/Distance_de_Levenshtein))
- [x] Jaro / Jaro-Winkler ([Explanation](https://fr.wikipedia.org/wiki/Distance_de_Jaro-Winkler))

### Phonetics
- [x] Soundex ([Explanation](https://en.wikipedia.org/wiki/Soundex))
- [x] Metaphone ([Explanation](https://en.wikipedia.org/wiki/Metaphone))
- [x] Double-metaphone ([Explanation](https://en.wikipedia.org/wiki/Metaphone#Double_Metaphone))
- [ ] Caverphone ([Explanation](https://en.wikipedia.org/wiki/Caverphone))
- [ ] Beider–Morse Phonetic ([Explanation](https://en.wikipedia.org/wiki/Daitch%E2%80%93Mokotoff_Soundex#Beider.E2.80.93Morse_Phonetic_Name_Matching_Algorithm))
- [ ] Kölner Phonetik ([Explanation](https://de.wikipedia.org/wiki/K%C3%B6lner_Phonetik))
- [ ] NYSIIS ([Explanation](https://en.wikipedia.org/wiki/New_York_State_Identification_and_Intelligence_System))

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

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
