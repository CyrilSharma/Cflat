use crate::parser::moduleParser;
use crate::printer::Printer;

#[test]
fn test0() {
    assert!(moduleParser::new().parse("22").is_err());
    assert!(moduleParser::new().parse("void thing() {}").is_ok());
    assert!(moduleParser::new().parse("int thing() { int x; }").is_ok());
    assert!(moduleParser::new().parse("int thing() { if (1) {} }").is_ok());
}

#[test]
fn visualize() {
    let m = moduleParser::new().parse("void thing() {}").unwrap();
    Printer::new().print_module(&m);
}