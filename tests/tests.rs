use compiler::asm;
use compiler::ast;
use compiler::ir;
use compiler::registry::Registry;

use asm::allocate               as AsmAllocate;
use asm::allocate::print_graph  as PrintInterference;
use asm::translator::Translator as AsmTranslator;
use asm::cfg::CFG               as AsmCfg;
use asm::cfgprinter::Printer    as AsmCfgPrinter;
use asm::liveness::Liveness     as AsmLiveness;
use asm::printer::Printer       as AsmPrinter;
use ast::analyzer::Analyzer     as AstAnalyzer;
use ast::printer::Printer       as AstPrinter;
use ir::translator::Translator  as IrTranslator;
use ir::printer::Printer        as IrPrinter;
use ir::reducer::Reducer        as IrReducer;
use ir::cfgbuilder::build       as IrCfgBuild;
use ir::cfgexporter::export     as IrCfgExport;
use ir::cfgprinter::Printer     as IrCfgPrinter;
use ir::reorder::reorder        as IrCfgReorder;
use lalrpop_util::ParseError;

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
    asm1cfg: bool,
    live:    bool,
    inter:   bool,
    coalasm: bool,
    coalint: bool,
    coalcfg: bool,
    asm2:    bool
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
        ir3:     true,
        ir3cfg:  false,
        frames:  false,
        asm1:    false,
        asm1cfg: false,
        live:    false,
        inter:   false,
        coalasm: true,
        coalint: false,
        coalcfg: false,
        asm2:    true,
    };
    while Path::new(&format!("{dir}/input{i}.c")).exists() {
        if i != 6 { i += 1; continue }
        let filepath = &format!("{dir}/input{i}.c");
        let input = fs::read_to_string(filepath).expect("File not found!");
        println!("{}", &format!("FILE: {filepath}"));
        let mut r = Registry::new();

        // Some exceptionally basic error handling.
        let res = ast::parser::moduleParser::new().parse(&input);
        let mut ast = match res {
            Ok(a) => a,
            Err(ParseError::InvalidToken { location }) => {
                println!("Invalid Token");
                let mut counter = 0;
                let mut lineidx = 0;
                let lines: Vec<String> = input.lines()
                    .map(|x| x.to_string()).collect();
                while counter <= location &&
                    lineidx < lines.len() {
                    counter += lines[lineidx].len();
                    lineidx += 1;
                }
                println!("{}: {}", lineidx, lines[lineidx - 1]);
                let linepos = location - (counter - lines[lineidx - 1].len()) + 3;
                println!("{}^", "-".repeat(linepos));
                return;
            },
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
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

        let order = IrCfgReorder(&cfg);
        let fir = IrCfgExport(cfg, order);
        if p.ir3 { IrPrinter::new().print(&fir); }

        if p.ir3cfg {
            let cfg = IrCfgBuild(&mut r, fir.clone());
            ir::cfgprinter::Printer::new().print(&cfg);
        }

        let asm = AsmTranslator::translate(&mut r, frames, fir);
        if p.asm1 { AsmPrinter::print(&asm); }
        

        let cfg = AsmCfg::build(&mut r, &asm);
        if p.asm1cfg { AsmCfgPrinter::print(&cfg); }

        if p.live {
            let cfg = AsmCfg::build(&mut r, &asm);
            let liveness = AsmLiveness::compute(cfg);
            let mut asm = Vec::new();
            for (a, _, _) in liveness { asm.push(a); }
            AsmPrinter::print_raw(&asm);
        }

        if p.inter {
            let cfg = AsmCfg::build(&mut r, &asm);
            let liveness = AsmLiveness::compute(cfg);
            let (_, alist) = AsmAllocate::build_graph(
                r.nids, &liveness
            );
            PrintInterference(alist);
        }

        if p.coalcfg || p.coalasm || p.coalint {
            let cfg = AsmCfg::build(&mut r, &asm);
            let mut liveness = AsmLiveness::compute(cfg);
            let (mut amat, mut alist) = AsmAllocate::build_graph(
                r.nids, &liveness
            );
            AsmAllocate::coalesce_graph(
                &mut liveness, &mut alist, &mut amat
            );
            let mut asm = Vec::new();
            for (a, _, _) in liveness { asm.push(a); }
            let cfg = AsmCfg::build(&mut r, &asm);
            if p.coalint {
                PrintInterference(alist);
            }
            if p.coalasm {
                AsmPrinter::print_raw(&asm);
            }
            if p.coalcfg {
                AsmCfgPrinter::print(&cfg);
            }
        }
        
        let liveness = AsmLiveness::compute(cfg);
        let asm = AsmAllocate::allocate(&mut r, liveness);
        if p.asm2 { AsmPrinter::print(&asm); }

        println!("\n\n\n\n\n");
        i += 1;
    }
}