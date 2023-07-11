#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {}

#[test]
fn parser() {
    assert!(parser::TermParser::new().parse("22").is_ok());
    assert!(parser::TermParser::new().parse("(22)").is_ok());
    assert!(parser::TermParser::new().parse("((((22))))").is_ok());
    assert!(parser::TermParser::new().parse("((22)").is_err());
}
