use super::Entry;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "table.pest"]
pub struct TableParser;

fn parse_hex<'a>(pair: Pair<'a, Rule>) -> u32 {
    u32::from_str_radix(pair.as_str(), 16).unwrap()
}

fn parse_entry<'a>(pair: Pair<'a, Rule>) -> Entry<'a> {
    let mut children = pair.into_inner();
    let codepoint = parse_hex(children.next().unwrap());
    let property = children.skip(1).next().unwrap().as_str();
    Entry {
        range: codepoint..=codepoint,
        property,
    }
}

pub fn parse<'a>(content: &'a str) -> Vec<Entry<'a>> {
    let mut entries = TableParser::parse(Rule::file, content)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .into_inner();
    entries.next_back();
    entries.map(parse_entry).collect()
}
