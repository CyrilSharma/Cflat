use compiler::asm;
use compiler::ast;
use compiler::ir;
use compiler::registry::Registry;

use std::fs;
use std::path::Path;

struct PrintConfig {
    ast1:    bool,
    ast2:    bool,
    ir1:     bool,
    ir2:     bool,
    ir2cfg:  bool,
    ir3:     bool,
    ir3cfg:  bool,
    frames:  bool,
    asm1:    bool,
    asm1cfg: bool
}

#[test]
fn visualize() {
    let mut i = 0;
    let dir = "tests/data";
    let p = PrintConfig {
        ast1:    false,
        ast2:    false,
        ir1:     false,
        ir2:     false,
        ir2cfg:  false,
        ir3:     false,
        ir3cfg:  false,
        frames:  false,
        asm1:    false,
        asm1cfg: false
    };
    while Path::new(&format!("{dir}/input{i}.c")).exists() {
        let filepath = &format!("{dir}/input{i}.c");
        let input = fs::read_to_string(filepath).expect("File not found!");

        println!("{}", &format!("FILE: {filepath}"));
        let mut r = Registry::new();

        let mut ast = ast::parser::moduleParser::new()
            .parse(&input)
            .expect("Parse Error!");
        if p.ast1 { ast::printer::Printer::new().print(&ast); }

        ast::analyzer::Analyzer::new(&mut r).analyze(&mut ast);
        if p.ast2 { ast::Printer::new().print(&ast); }

        let ir  = ir::translator::Translator::new(&mut r).translate(&mut ast);
        if p.ir1 { ir::printer::Printer::new().print(&ir); }

        let lir = ir::Reducer::new(&mut r).reduce(ir);
        if p.ir2 { ir::printer::Printer::new().print(&lir); }

        let cfg = ir::cfgbuilder::build(&mut r, lir);
        if p.ir2cfg { ir::cfgprinter::Printer::new().print(&cfg); }

        let frames = ir::cfgframer::Framer::new(&mut r, &cfg).frame();
        if p.frames {
            println!("Frames - ");
            for (id, loc) in frames.iter().enumerate() {
                if *loc == usize::MAX { continue };
                println!("  {id}: {loc}");
            }
            println!("");
        }

        let order: Vec<usize> = (0..cfg.nodes.len()).collect();
        let fir = ir::cfgexporter::export(cfg, order);
        if p.ir3 { ir::printer::Printer::new().print(&fir); }
        if p.ir3cfg {
            let cfg = ir::cfgbuilder::build(&mut r, fir.clone());
            cfgprinter::Printer::new().print(&cfg);
        }

        let asm = asm::translator::Translator::new(&r, frames).translate(fir);
        if p.asm1 { asm::printer::Printer::new().print(&asm); }
        println!("\n\n\n\n\n");
        i += 1;
    }
}