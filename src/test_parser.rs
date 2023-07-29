use std::fs;
use std::path::Path;
use crate::parser::moduleParser;
use crate::astprinter;

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
    let mut i = 0;
    let dir = "tests/data/";
    while Path::new(&format!("{dir}/input{i}.c")).exists() {
        let filepath = &format!("{dir}/input{i}.c");
        let input = fs::read_to_string(filepath).expect("File not found!");
        let m = moduleParser::new().parse(&input).expect("Parse Error!");
        astprinter::Printer::new().print(&m);
        i += 1
    }
}