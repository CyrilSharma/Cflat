#[allow(unused_imports)]
use compiler::asmtranslator;
use compiler::asmprinter;
use compiler::astanalyzer::Analyzer;
use compiler::astparser::moduleParser;
use compiler::astprinter;
use compiler::cfgbuilder::build;
use compiler::cfgframer::Framer;
use compiler::cfgprinter;
use compiler::cfgexporter::export;
use compiler::irprinter;
use compiler::irtranslator;
use compiler::irreducer::Reducer;
use compiler::registry::Registry;

use std::fs;
use std::path::Path;

struct PrintConfig {
    ast1:   bool,
    ast2:   bool,
    ir1:    bool,
    ir2:    bool,
    ir2cfg: bool,
    ir3:    bool,
    ir3cfg: bool,
    frames: bool,
    asm1:   bool
}

#[test]
fn visualize() {
    let mut i = 0;
    let dir = "tests/data";
    let p = PrintConfig {
        ast1:   false,
        ast2:   false,
        ir1:    false,
        ir2:    false,
        ir2cfg: false,
        ir3:    false,
        ir3cfg: true,
        frames: false,
        asm1:   false,
    };
    while Path::new(&format!("{dir}/input{i}.c")).exists() {
        let filepath = &format!("{dir}/input{i}.c");
        let input = fs::read_to_string(filepath).expect("File not found!");

        println!("{}", &format!("FILE: {filepath}"));
        let mut r = Registry::new();

        let mut ast = moduleParser::new().parse(&input).expect("Parse Error!");
        if p.ast1 { astprinter::Printer::new().print(&ast); }

        Analyzer::new(&mut r).analyze(&mut ast);
        if p.ast2 { astprinter::Printer::new().print(&ast); }

        let ir  = irtranslator::Translator::new(&mut r).translate(&mut ast);
        if p.ir1 { irprinter::Printer::new().print(&ir); }

        let lir = Reducer::new(&mut r).reduce(ir);
        if p.ir2 { irprinter::Printer::new().print(&lir); }

        let cfg = build(&mut r, lir);
        if p.ir2cfg { cfgprinter::Printer::new().print(&cfg); }


        let frames = Framer::new(&mut r, &cfg).frame();
        if p.frames {
            println!("Frames - ");
            for (id, loc) in frames.iter().enumerate() {
                if *loc == usize::MAX { continue };
                println!("  {id}: {loc}");
            }
            println!("");
        }

        let order: Vec<usize> = (0..cfg.nodes.len()).collect();
        let fir = export(cfg, order);
        if p.ir3 { irprinter::Printer::new().print(&fir); }
        if p.ir3cfg {
            let cfg = build(&mut r, fir.clone());
            cfgprinter::Printer::new().print(&cfg);
        }

        let asm = asmtranslator::Translator::new(&r, frames).translate(fir);
        if p.asm1 { asmprinter::Printer::new().print(&asm); }
        println!("\n\n\n\n\n");
        i += 1;
    }
}