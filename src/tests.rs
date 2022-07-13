use crate::parse::*;

#[test]
fn valid_bad1() {
    let md_string = "- [ ] 1 + 1 = 3.".to_string();
    assert!(parse(md_string).is_ok());
}

#[test]
fn valid_good1() {
    let md_string = "- [ ] 1 + 1 = 3.".to_string();
    assert!(parse(md_string).is_ok());
}

#[test]
fn valid_good2() {
    let md_string = "- [x] 1 * 1 = 1.".to_string();
    assert!(parse(md_string).is_ok());
}

#[test]
fn valid_good3() {
    let md_string = "what is 2 + 2? \
- [x] 4.
- [ ] 3.
- [x] -2 x -2.
"
    .to_string();
    assert!(parse(md_string).is_ok());
}

#[test]
#[should_panic]
fn invalid() {
    let md_string = "what is 2 + 2?".to_string();
    assert!(parse(md_string).is_ok());
}
