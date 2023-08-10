#[allow(unused_imports)]
use compiler::astprinter;
use compiler::cfgprinter;
use compiler::irprinter;
use compiler::irtranslator::Translator;
use compiler::irreducer::Reducer;
use compiler::astparser::moduleParser;
use compiler::registry::Registry;
use compiler::astanalyzer::Analyzer;

use std::fs;
use std::path::Path;

use bumpalo::Bump;

#[test]
fn visualize() {
    let mut i = 0;
    let dir = "tests/data";
    while Path::new(&format!("{dir}/input{i}.c")).exists() {
        let filepath = &format!("{dir}/input{i}.c");
        let input = fs::read_to_string(filepath).expect("File not found!");

        println!("{}", &format!("FILE: {filepath}"));
        let mut r = Registry::new();

        // Might make these files use bump allocator as well.
        let mut ast = moduleParser::new().parse(&input).expect("Parse Error!");
        astprinter::Printer::new().print(&ast);
        let analyzer = Analyzer::new(&mut r).analyze(&mut ast);
        astprinter::Printer::new().print(&ast);

        let (mut old, mut cur) = (Bump::new(), Bump::new());
        let ir  = Translator::new(cur).translate(&mut ast);
        irprinter::Printer::new().print(&ir);
        old = std::mem::take(&mut cur);

        let lir = Reducer::new(cur, &mut r).reduce(&ir);
        irprinter::Printer::new().print(&lir);
        old = std::mem::take(&mut cur);

        let cfg = CfgBuilder::new(cur).build(&lir);
        cfgprinter::Printer::new().print(&cfg);
        old = std::mem::take(&mut cur);
        println!("\n\n\n\n\n");
        i += 1;
    }
}