use clap::Parser;
use convert_case::{Case, Casing};
use regex::Regex;
use std::{io::Write, path::PathBuf};
use typst::syntax::SyntaxNode;

fn print_s_expression(node: &SyntaxNode, indent: usize, span: bool, w: &mut impl Write) {
    for _ in 0..indent {
        write!(w, "  ").unwrap();
    }
    write!(w, "{}", format!("({:?}", node.kind()).to_case(Case::Snake)).unwrap();
    if span {
        if !node.text().is_empty() {
            write!(w, " {:?}", node.text()).unwrap();
        }
    }
    for child in node.children() {
        writeln!(w).unwrap();
        print_s_expression(child, indent + 1, span, w);
    }
    write!(w, ")").unwrap();
}

fn print_test(name: &str, string: &str, span: bool, w: &mut impl Write) {
    writeln!(w, "====================").unwrap();
    writeln!(w, "{}", name).unwrap();
    writeln!(w, "====================").unwrap();
    writeln!(w, "{}", string).unwrap();
    writeln!(w, "--------------------").unwrap();
    writeln!(w,).unwrap();
    let tree: SyntaxNode = typst::syntax::parse(string);
    writeln!(w, "(source_file").unwrap();
    print_s_expression(&tree, 1, span, w);
    writeln!(w, ")").unwrap();
    writeln!(w,).unwrap();
    writeln!(w,).unwrap();
}

#[derive(Parser)]
struct Args {
    write_to: PathBuf,
}

fn main() {
    let Args { write_to } = Args::parse();
    let sep = Regex::new("\n?(={20}|-{20})\n").unwrap();
    let content = std::fs::read_to_string("tests.txt").unwrap();
    let mut parts = sep.split(&content);
    let mut file = std::fs::File::create(&write_to).unwrap();
    let mut stdout = std::io::stdout();
    while let (Some(_), Some(name), Some(string)) = (parts.next(), parts.next(), parts.next()) {
        print_test(name, string, true, &mut stdout);
        print_test(name, string, false, &mut file);
    }
}
