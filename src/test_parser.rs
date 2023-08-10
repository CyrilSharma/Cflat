use crate::astparser::moduleParser;

#[test]
fn test0() {
    assert!(moduleParser::new().parse("22").is_err());
    assert!(moduleParser::new().parse("void thing() {}").is_ok());
    assert!(moduleParser::new().parse("int thing() { int x; }").is_ok());
    assert!(moduleParser::new().parse("int thing() { if (1) {} }").is_ok());
}