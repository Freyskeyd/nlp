extern crate nlp;
use nlp::tokenize::tokenize;

#[test]
fn test_tokenize() {
    assert_eq!(tokenize("hello, world!"), vec!["hello", "world"]);
}
