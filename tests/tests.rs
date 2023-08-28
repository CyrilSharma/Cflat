use compiler::asm;
use compiler::ast;
use compiler::ir;
use compiler::registry::Registry;

use asm::translator::Translator as AsmTranslator;
use asm::cfg::CFG               as AsmCfg;
use asm::cfgprinter::Printer    as AsmCfgPrinter;
use asm::printer::Printer       as AsmPrinter;
use ast::analyzer::Analyzer     as AstAnalyzer;
use ast::printer::Printer       as AstPrinter;
use ir::translator::Translator  as IrTranslator;
use ir::printer::Printer        as IrPrinter;
use ir::reducer::Reducer        as IrReducer;
use ir::cfgbuilder::build       as IrCfgBuild;
use ir::cfgexporter::export     as IrCfgExport;
use ir::cfgprinter::Printer     as IrCfgPrinter;

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
        asm1cfg: true
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

        AstAnalyzer::new(&mut r).analyze(&mut ast);
        if p.ast2 { AstPrinter::new().print(&ast); }

        
        let ir  = IrTranslator::new(&mut r).translate(&mut ast);
        if p.ir1 { IrPrinter::new().print(&ir); }

        
        let lir = IrReducer::new(&mut r).reduce(ir);
        if p.ir2 { IrPrinter::new().print(&lir); }

        let cfg = IrCfgBuild(&mut r, lir);
        if p.ir2cfg { IrCfgPrinter::new().print(&cfg); }

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
        let fir = IrCfgExport(cfg, order);
        if p.ir3 { IrPrinter::new().print(&fir); }
        if p.ir3cfg {
            let cfg = IrCfgBuild(&mut r, fir.clone());
            ir::cfgprinter::Printer::new().print(&cfg);
        }

        let asm = AsmTranslator::new(&r, frames).translate(fir);
        if p.asm1 { AsmPrinter::new().print(&asm); }
        let cfg = AsmCfg::build(&mut r, asm);
        if p.asm1cfg { AsmCfgPrinter::new().print(&cfg) }
        println!("\n\n\n\n\n");
        i += 1;
    }
}