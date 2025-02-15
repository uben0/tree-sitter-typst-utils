use clap::Parser;
use convert_case::{Case, Casing};
use regex::Regex;
use std::{io::Write, path::PathBuf};
use typst::syntax::{SyntaxKind, SyntaxNode};

struct Custom {
    kind: SyntaxKind,
    children: Vec<Box<dyn WriteTree>>,
}
impl From<SyntaxKind> for Custom {
    fn from(value: SyntaxKind) -> Self {
        Custom {
            kind: value,
            children: Vec::new(),
        }
    }
}
impl Custom {
    fn new<const N: usize>(kind: SyntaxKind, children: [Box<dyn WriteTree>; N]) -> Self {
        Self {
            kind,
            children: children.into(),
        }
    }
}

trait WriteTree {
    fn write_tree(&self, indent: usize, span: bool, w: &mut dyn Write);
}

impl WriteTree for Custom {
    fn write_tree(&self, indent: usize, span: bool, w: &mut dyn Write) {
        writeln!(w).unwrap();
        for _ in 0..indent {
            write!(w, "  ").unwrap();
        }
        write!(w, "{}", format!("({:?}", self.kind).to_case(Case::Snake)).unwrap();
        for child in &self.children {
            child.write_tree(indent + 1, span, w);
        }
        write!(w, ")").unwrap();
    }
}

impl WriteTree for SyntaxNode {
    fn write_tree(&self, indent: usize, span: bool, w: &mut dyn Write) {
        match self.kind() {
            SyntaxKind::Raw => {
                Custom::new(
                    SyntaxKind::Raw,
                    [
                        Box::new(Custom::from(SyntaxKind::RawDelim)),
                        Box::new(Custom::from(SyntaxKind::Text)),
                        Box::new(Custom::from(SyntaxKind::RawDelim)),
                    ],
                )
                .write_tree(indent, span, w);
                return;
            }
            _ => {}
        }
        writeln!(w).unwrap();
        for _ in 0..indent {
            write!(w, "  ").unwrap();
        }
        write!(w, "{}", format!("({:?}", self.kind()).to_case(Case::Snake)).unwrap();
        if span {
            if !self.text().is_empty() {
                write!(w, " {:?}", self.text()).unwrap();
            }
        }
        for child in self.children() {
            child.write_tree(indent + 1, span, w);
        }
        write!(w, ")").unwrap();
    }
}

// struct Custom {
//     custom: SyntaxKind,
//     children: Vec<Self>,
// }

// fn print_s_expression(node: &SyntaxNode, indent: usize, span: bool, w: &mut impl Write) {
//     match node.kind() {
//         SyntaxKind::Raw => {}
//         SyntaxKind::RawTrimmed => {
//             return;
//         }
//         _ => {}
//     }
//     writeln!(w).unwrap();
//     for _ in 0..indent {
//         write!(w, "  ").unwrap();
//     }
//     write!(w, "{}", format!("({:?}", node.kind()).to_case(Case::Snake)).unwrap();
//     if span {
//         if !node.text().is_empty() {
//             write!(w, " {:?}", node.text()).unwrap();
//         }
//     }
//     for child in node.children() {
//         print_s_expression(child, indent + 1, span, w);
//     }
//     write!(w, ")").unwrap();
// }

fn print_test(name: &str, string: &str, span: bool, w: &mut impl Write) {
    writeln!(w, "====================").unwrap();
    writeln!(w, "{}", name).unwrap();
    writeln!(w, "====================").unwrap();
    writeln!(w, "{}", string).unwrap();
    writeln!(w, "--------------------").unwrap();
    writeln!(w,).unwrap();
    let tree: SyntaxNode = typst::syntax::parse(string);
    write!(w, "(source_file").unwrap();
    tree.write_tree(1, span, w);
    // print_s_expression(&tree, 1, span, w);
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
