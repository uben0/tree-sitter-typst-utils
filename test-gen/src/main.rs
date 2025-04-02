use clap::Parser;
use convert_case::{Case, Casing};
use regex::Regex;
use std::{io::Write, path::PathBuf};
use typst::syntax::{SyntaxKind, SyntaxNode};

mod schemer;
use schemer::{Writtable, Writter};

struct Token(&'static str);
const RAW_DELIM: Token = Token("raw_delim");
const RAW_BLOB: Token = Token("raw_blob");
const RAW_LANG: Token = Token("raw_lang");

impl Writtable for Token {
    fn write<W: Write>(self, w: Writter<W>) -> Writter<W> {
        let Self(token) = self;
        w.node(token, |w| w)
    }
}
impl Writtable for &SyntaxNode {
    fn write<W: Write>(self, w: Writter<W>) -> Writter<W> {
        match self.kind() {
            SyntaxKind::Raw => w.node("raw", |w| {
                w.line()
                    .param(RAW_DELIM)
                    .fold(
                        self.children()
                            .nth(1)
                            .filter(|n| n.kind() == SyntaxKind::RawLang)
                            .map(|_| RAW_LANG),
                        |w, n| w.line().param(n),
                    )
                    .line()
                    .param(RAW_BLOB)
                    .line()
                    .param(RAW_DELIM)
            }),
            kind => w.node(&format!("{:?}", kind).to_case(Case::Snake), |w| {
                w.fold(self.children(), |w, child| w.line().param(child))
            }),
        }
    }
}

fn print_test(name: &str, string: &str, mut w: &mut impl Write, color: bool) {
    writeln!(w, "====================").unwrap();
    writeln!(w, "{}", name).unwrap();
    writeln!(w, "====================").unwrap();
    writeln!(w, "{}", string).unwrap();
    writeln!(w, "--------------------").unwrap();
    writeln!(w).unwrap();
    let tree: SyntaxNode = typst::syntax::parse(string);
    Writter::root(
        &mut w,
        |w| w.node("source_file", |w| w.line().param(&tree)),
        color,
    );
    writeln!(w).unwrap();
    writeln!(w).unwrap();
    writeln!(w).unwrap();
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
        print_test(name, string, &mut stdout, true);
        print_test(name, string, &mut file, false);
    }
}
