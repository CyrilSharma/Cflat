use std::fs;
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
#[allow(dead_code)]
fn visualize() {
    let path0 = "tests/data/parser/input0.c";
    let input0 = fs::read_to_string(path0).expect("File not found!");
    let m = moduleParser::new().parse(&input0).expect("Parse Error!");
    Printer::new().print_module(&m);

    let path1 = "tests/data/parser/input1.c";
    let input1 = fs::read_to_string(path1).expect("File not found!");
    let m = moduleParser::new().parse(&input1).expect("Parse Error!");
    Printer::new().print_module(&m);
}